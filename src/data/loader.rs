use anyhow::{Context, Result};
use polars::prelude::*;
use std::path::Path;


pub struct LoadOptions {
    pub streaming: bool,
    pub infer_schema_length: Option<usize>,
    pub has_header: bool,
    pub try_parse_dates: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            streaming: false,
            infer_schema_length: Some(1000),
            has_header: true,
            try_parse_dates: true,
        }
    }
}

pub fn load_csv(path: &Path, options: &LoadOptions) -> Result<LazyFrame> {
    // Load CSV with proper error handling
    let df = CsvReader::from_path(path)
        .with_context(|| format!("Failed to open CSV file: {}", path.display()))?
        .has_header(options.has_header)
        .infer_schema(options.infer_schema_length)
        .with_try_parse_dates(options.try_parse_dates)
        .finish()
        .with_context(|| format!("Failed to parse CSV file: {}", path.display()))?;
    
    let lf = df.lazy();

    // Apply date format detection and parsing for common patterns
    detect_and_parse_dates(lf)
}

fn detect_and_parse_dates(lf: LazyFrame) -> Result<LazyFrame> {
    // Get column information to detect date patterns
    let schema = lf.schema().map_err(|e| anyhow::anyhow!("Failed to get schema: {}", e))?;
    let mut result = lf;
    
    // Look for common date patterns in string columns
    for (col_name, dtype) in schema.iter() {
        if matches!(dtype, DataType::Utf8) {
            // Check if this looks like a date column
            if is_likely_date_column(col_name) {
                // Sample the data to detect format
                if let Ok(detected_format) = detect_date_format(&result, col_name) {
                    result = try_parse_date_column(result, col_name, &detected_format)?;
                }
            }
        }
        // Handle timestamp columns (likely microseconds since epoch)
        else if matches!(dtype, DataType::Int64) && is_likely_timestamp_column(col_name) {
            result = try_parse_timestamp_column(result, col_name)?;
        }
    }
    
    Ok(result)
}

fn is_likely_date_column(col_name: &str) -> bool {
    let date_patterns = [
        "date", "time", "timestamp", "created", "updated", "modified",
        "event_date", "session_date", "first_seen", "last_seen"
    ];
    
    let col_lower = col_name.to_lowercase();
    date_patterns.iter().any(|pattern| col_lower.contains(pattern))
}

fn is_likely_timestamp_column(col_name: &str) -> bool {
    let timestamp_patterns = [
        "timestamp", "_timestamp", "time_micros", "event_timestamp"
    ];
    
    let col_lower = col_name.to_lowercase();
    timestamp_patterns.iter().any(|pattern| col_lower.contains(pattern))
}

#[derive(Debug, Clone)]
enum DateFormat {
    Iso,        // YYYY-MM-DD
    IsoDateTime, // YYYY-MM-DD HH:MM:SS
    YyyyMmDd,   // YYYYMMDD
    MmDdYyyy,   // MM/DD/YYYY
    DdMmYyyy,   // DD/MM/YYYY
}

impl DateFormat {
    fn to_polars_format(&self) -> &'static str {
        match self {
            DateFormat::Iso => "%Y-%m-%d",
            DateFormat::IsoDateTime => "%Y-%m-%d %H:%M:%S",
            DateFormat::YyyyMmDd => "%Y%m%d",
            DateFormat::MmDdYyyy => "%m/%d/%Y",
            DateFormat::DdMmYyyy => "%d/%m/%Y",
        }
    }
}

fn detect_date_format(lf: &LazyFrame, col_name: &str) -> Result<DateFormat> {
    // Collect a few sample values to detect the format
    let sample_df = lf.clone()
        .select([col(col_name)])
        .limit(10)
        .collect()
        .map_err(|e| anyhow::anyhow!("Failed to sample data for date detection: {}", e))?;
    
    let column = sample_df.column(col_name)
        .map_err(|e| anyhow::anyhow!("Column '{}' not found in sample: {}", col_name, e))?;
    
    // Get the first non-null string value
    for i in 0..column.len() {
        if let Ok(AnyValue::Utf8(date_str)) = column.get(i) {
            return detect_format_from_string(date_str);
        }
    }
    
    anyhow::bail!("Could not find valid date string in column '{}'", col_name)
}

fn detect_format_from_string(date_str: &str) -> Result<DateFormat> {
    let trimmed = date_str.trim();
    
    // Check for YYYY-MM-DD HH:MM:SS (ISO datetime)
    if trimmed.len() >= 19 && trimmed.chars().nth(4) == Some('-') && trimmed.chars().nth(7) == Some('-') && trimmed.chars().nth(10) == Some(' ') {
        return Ok(DateFormat::IsoDateTime);
    }
    
    // Check for YYYY-MM-DD (ISO date)
    if trimmed.len() == 10 && trimmed.chars().nth(4) == Some('-') && trimmed.chars().nth(7) == Some('-') {
        return Ok(DateFormat::Iso);
    }
    
    // Check for YYYYMMDD
    if trimmed.len() == 8 && trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Ok(DateFormat::YyyyMmDd);
    }
    
    // Check for MM/DD/YYYY or DD/MM/YYYY
    if trimmed.len() == 10 && trimmed.chars().nth(2) == Some('/') && trimmed.chars().nth(5) == Some('/') {
        // This is ambiguous - we'll default to MM/DD/YYYY (US format)
        // In a real implementation, you might want to make this configurable
        return Ok(DateFormat::MmDdYyyy);
    }
    
    anyhow::bail!("Could not detect date format for string: '{}'", date_str)
}

fn try_parse_date_column(lf: LazyFrame, col_name: &str, format: &DateFormat) -> Result<LazyFrame> {
    let parsed_col_name = format!("{}_parsed", col_name);
    
    // For Polars 0.35, we'll use a simpler approach
    // In a newer version, we could use strptime more directly
    let result = match format {
        DateFormat::Iso | DateFormat::IsoDateTime => {
            // Polars should auto-detect ISO format
            lf.with_columns([
                col(col_name)
                    .cast(DataType::Date)
                    .alias(&parsed_col_name)
            ])
        }
        _ => {
            // For other formats, we'll keep the original for now
            // TODO: Implement custom parsing for non-ISO formats
            lf.with_columns([
                col(col_name)
                    .alias(&parsed_col_name)
            ])
        }
    };
    
    Ok(result)
}

fn try_parse_timestamp_column(lf: LazyFrame, col_name: &str) -> Result<LazyFrame> {
    let parsed_col_name = format!("{}_parsed", col_name);
    
    // Convert microseconds since epoch to datetime
    // For Polars 0.35, we'll use a simpler approach
    let result = lf.with_columns([
        // Convert microseconds to milliseconds and then to datetime
        col(col_name)
            .cast(DataType::Int64)
            .floor_div(lit(1_000)) // Convert microseconds to milliseconds
            .cast(DataType::Datetime(TimeUnit::Milliseconds, None))
            .alias(&parsed_col_name)
    ]);
    
    Ok(result)
}

pub fn validate_columns(lf: &LazyFrame, required_columns: &[String]) -> Result<()> {
    let schema = lf.schema().map_err(|e| anyhow::anyhow!("Failed to get schema: {}", e))?;
    let available_columns: Vec<String> = schema.iter_names().map(|s| s.to_string()).collect();
    
    for required_col in required_columns {
        if !available_columns.contains(required_col) {
            let suggestion = suggest_column_name(&available_columns, required_col);
            match suggestion {
                Some(suggested) => {
                    anyhow::bail!(
                        "Column '{}' not found in CSV. Available columns: {:?}\nDid you mean '{}'?",
                        required_col,
                        available_columns,
                        suggested
                    );
                }
                None => {
                    anyhow::bail!(
                        "Column '{}' not found in CSV. Available columns: {:?}",
                        required_col,
                        available_columns
                    );
                }
            }
        }
    }
    
    Ok(())
}

pub fn suggest_column_name(available: &[String], requested: &str) -> Option<String> {
    let requested_lower = requested.to_lowercase();
    
    // First, try exact case-insensitive match
    for col in available {
        if col.to_lowercase() == requested_lower {
            return Some(col.clone());
        }
    }
    
    // Then try partial matches
    for col in available {
        let col_lower = col.to_lowercase();
        if col_lower.contains(&requested_lower) || requested_lower.contains(&col_lower) {
            return Some(col.clone());
        }
    }
    
    // Finally, try fuzzy matching using Levenshtein distance
    let mut best_match = None;
    let mut best_distance = usize::MAX;
    
    for col in available {
        let distance = levenshtein_distance(&requested_lower, &col.to_lowercase());
        if distance < best_distance && distance <= 3 { // Only suggest if distance is reasonable
            best_distance = distance;
            best_match = Some(col.clone());
        }
    }
    
    best_match
}

// Simple Levenshtein distance implementation
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }
    
    matrix[len1][len2]
}

pub fn get_column_names(lf: &LazyFrame) -> Result<Vec<String>> {
    let schema = lf.schema().map_err(|e| anyhow::anyhow!("Failed to get schema: {}", e))?;
    Ok(schema.iter_names().map(|s| s.to_string()).collect())
}
