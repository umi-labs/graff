use anyhow::Result;
use polars::prelude::*;
use std::collections::HashMap;

pub fn apply_derived_columns(
    lf: LazyFrame,
    derivations: &HashMap<String, String>,
) -> Result<LazyFrame> {
    let mut result = lf;

    for (col_name, expr_str) in derivations {
        let derived_expr = parse_derive_expression(expr_str)?;
        result = result.with_columns([derived_expr.alias(col_name)]);
    }

    Ok(result)
}

fn parse_derive_expression(expr: &str) -> Result<Expr> {
    match expr {
        s if s.starts_with("to_week(") => {
            let col_name = extract_column_name(s)?;
            Ok(to_week_expr(col_name))
        }
        s if s.starts_with("to_month(") => {
            let col_name = extract_column_name(s)?;
            Ok(to_month_expr(col_name))
        }
        s if s.starts_with("to_hour(") => {
            let col_name = extract_column_name(s)?;
            Ok(to_hour_expr(col_name))
        }
        s if s.starts_with("weekday(") => {
            let col_name = extract_column_name(s)?;
            Ok(weekday_expr(col_name))
        }
        s if s.starts_with("source_medium(") => {
            let (source_col, medium_col) = extract_two_column_names(s)?;
            Ok(source_medium_expr(source_col, medium_col))
        }
        _ => {
            // TODO: Implement more complex expression parsing
            anyhow::bail!("Unsupported derive expression: {}", expr)
        }
    }
}

fn extract_column_name(expr: &str) -> Result<&str> {
    let start = expr.find('(').unwrap() + 1;
    let end = expr.rfind(')').unwrap();
    Ok(&expr[start..end])
}

fn extract_two_column_names(expr: &str) -> Result<(&str, &str)> {
    let start = expr.find('(').unwrap() + 1;
    let end = expr.rfind(')').unwrap();
    let inner = &expr[start..end];
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

    if parts.len() != 2 {
        anyhow::bail!("Expected two column names, got: {}", inner);
    }

    Ok((parts[0], parts[1]))
}

/// Convert date to Monday week start
fn to_week_expr(col_name: &str) -> Expr {
    col(col_name).dt().truncate(lit("1w"), "0".to_string())
}

/// Convert date to first of month
fn to_month_expr(col_name: &str) -> Expr {
    col(col_name).dt().truncate(lit("1mo"), "0".to_string())
}

/// Extract hour from timestamp (0-23)
fn to_hour_expr(col_name: &str) -> Expr {
    col(col_name).dt().hour()
}

/// Get day of week (0=Monday, 6=Sunday)
fn weekday_expr(col_name: &str) -> Expr {
    col(col_name).dt().weekday()
}

/// Combine source and medium as "source / medium"
fn source_medium_expr(source_col: &str, medium_col: &str) -> Expr {
    // For now, use format! to create a simple concatenation
    // TODO: Use proper polars string concatenation when available
    concat_expr([col(source_col), lit(" / "), col(medium_col)], false).unwrap_or_else(|_| {
        // Fallback: simple format string
        col(source_col)
    })
}
