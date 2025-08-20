use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChartSpec {
    pub data: Option<DataConfig>,
    pub charts: Vec<ChartConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataConfig {
    pub default: Option<PathBuf>,
    pub sources: Option<HashMap<String, PathBuf>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChartConfig {
    #[serde(rename = "type")]
    pub chart_type: ChartType,
    pub title: Option<String>,
    pub data: Option<PathBuf>,
    pub x: Option<String>,
    pub y: Option<String>,
    pub z: Option<String>, // For heatmaps
    pub group_by: Option<String>,
    pub agg: Option<AggregationType>,
    pub filter: Option<FilterConfig>,
    pub derive: Option<HashMap<String, String>>,
    pub sort: Option<Vec<SortConfig>>,
    pub limit: Option<usize>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub theme: Option<Theme>,
    pub format: Option<OutputFormat>,
    pub scale: Option<f64>,

    // Chart-specific fields
    pub stacked: Option<bool>,
    pub horizontal: Option<bool>,
    pub normalize: Option<bool>,
    pub bins: Option<u32>,
    pub colormap: Option<ColorMap>,
    pub steps: Option<Vec<String>>,
    pub step_order: Option<Vec<usize>>, // For funnel charts - order of steps
    pub value_labels: Option<ValueLabelPosition>, // For funnel charts - label position
    pub values: Option<String>,
    pub conversion_rates: Option<bool>,
    pub cohort_date: Option<String>,
    pub period_number: Option<String>,
    pub users: Option<String>,
    pub percentage: Option<bool>,
    pub legend_position: Option<LegendPosition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FilterConfig {
    pub include: Option<HashMap<String, FilterValue>>,
    pub exclude: Option<HashMap<String, FilterValue>>,
    pub expression: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FilterValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SortConfig {
    pub column: String,
    pub ascending: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ChartType {
    Line,
    Area,
    Bar,
    BarStacked,
    Heatmap,
    Scatter,
    Funnel,
    Retention,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AggregationType {
    Sum,
    Count,
    Mean,
    Median,
    Min,
    Max,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Png,
    Svg,
    Pdf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LegendPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Deserialize, Serialize, Clone, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ValueLabelPosition {
    Left,
    Right,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ColorMap {
    Viridis,
    Plasma,
    Blues,
    Reds,
    Greens,
}

impl ChartSpec {
    pub fn from_yaml(content: &str) -> anyhow::Result<Self> {
        let spec: Self = serde_yaml::from_str(content)?;
        spec.validate()?;
        Ok(spec)
    }

    pub fn from_json(content: &str) -> anyhow::Result<Self> {
        let spec: Self = serde_json::from_str(content)?;
        spec.validate()?;
        Ok(spec)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.charts.is_empty() {
            anyhow::bail!("Chart specification must contain at least one chart");
        }

        for (index, chart) in self.charts.iter().enumerate() {
            chart
                .validate()
                .with_context(|| format!("Chart {} validation failed", index + 1))?;
        }

        Ok(())
    }
}

impl ChartConfig {
    pub fn validate(&self) -> anyhow::Result<()> {
        // Validate required fields based on chart type
        match self.chart_type {
            ChartType::Heatmap => {
                if self.z.is_none() {
                    anyhow::bail!("Heatmap charts require a 'z' field for color intensity values");
                }
            }
            ChartType::Funnel => {
                if self.steps.is_none() {
                    anyhow::bail!("Funnel charts require a 'steps' field with step names");
                }
                if self.values.is_none() {
                    anyhow::bail!("Funnel charts require a 'values' field for step values");
                }
            }
            ChartType::Retention => {
                if self.cohort_date.is_none() {
                    anyhow::bail!("Retention charts require a 'cohort_date' field");
                }
                if self.period_number.is_none() {
                    anyhow::bail!("Retention charts require a 'period_number' field");
                }
                if self.users.is_none() {
                    anyhow::bail!("Retention charts require a 'users' field");
                }
            }
            _ => {
                // Line, Area, Bar, BarStacked, Scatter charts all require x and y
                if self.x.is_none() {
                    anyhow::bail!("{:?} charts require an 'x' field", self.chart_type);
                }
                if self.y.is_none() {
                    anyhow::bail!("{:?} charts require a 'y' field", self.chart_type);
                }
            }
        }

        // Validate dimensions
        if let Some(width) = self.width {
            if width < 100 || width > 10000 {
                anyhow::bail!(
                    "Chart width must be between 100 and 10000 pixels, got {}",
                    width
                );
            }
        }

        if let Some(height) = self.height {
            if height < 100 || height > 10000 {
                anyhow::bail!(
                    "Chart height must be between 100 and 10000 pixels, got {}",
                    height
                );
            }
        }

        // Validate scale
        if let Some(scale) = self.scale {
            if scale <= 0.0 || scale > 10.0 {
                anyhow::bail!("Chart scale must be between 0.1 and 10.0, got {}", scale);
            }
        }

        // Validate bins for heatmaps
        if let Some(bins) = self.bins {
            if bins < 2 || bins > 100 {
                anyhow::bail!("Heatmap bins must be between 2 and 100, got {}", bins);
            }
        }

        // Validate filter expressions
        if let Some(filter) = &self.filter {
            self.validate_filter(filter)?;
        }

        Ok(())
    }

    fn validate_filter(&self, filter: &FilterConfig) -> anyhow::Result<()> {
        // Validate that we have at least one filter condition
        let has_include = filter.include.as_ref().map_or(false, |f| !f.is_empty());
        let has_exclude = filter.exclude.as_ref().map_or(false, |f| !f.is_empty());
        let has_expression = filter.expression.is_some();

        if !has_include && !has_exclude && !has_expression {
            anyhow::bail!(
                "Filter configuration must have at least one condition (include, exclude, or expression)"
            );
        }

        // Validate filter values
        if let Some(include) = &filter.include {
            for (column, values) in include {
                if column.is_empty() {
                    anyhow::bail!("Filter column name cannot be empty");
                }
                match values {
                    FilterValue::Single(value) => {
                        if value.is_empty() {
                            anyhow::bail!("Filter value cannot be empty for column '{}'", column);
                        }
                    }
                    FilterValue::Multiple(values) => {
                        if values.is_empty() {
                            anyhow::bail!(
                                "Filter values list cannot be empty for column '{}'",
                                column
                            );
                        }
                        for value in values {
                            if value.is_empty() {
                                anyhow::bail!(
                                    "Filter value cannot be empty for column '{}'",
                                    column
                                );
                            }
                        }
                    }
                }
            }
        }

        if let Some(exclude) = &filter.exclude {
            for (column, values) in exclude {
                if column.is_empty() {
                    anyhow::bail!("Filter column name cannot be empty");
                }
                match values {
                    FilterValue::Single(value) => {
                        if value.is_empty() {
                            anyhow::bail!("Filter value cannot be empty for column '{}'", column);
                        }
                    }
                    FilterValue::Multiple(values) => {
                        if values.is_empty() {
                            anyhow::bail!(
                                "Filter values list cannot be empty for column '{}'",
                                column
                            );
                        }
                        for value in values {
                            if value.is_empty() {
                                anyhow::bail!(
                                    "Filter value cannot be empty for column '{}'",
                                    column
                                );
                            }
                        }
                    }
                }
            }
        }

        if let Some(expression) = &filter.expression {
            if expression.trim().is_empty() {
                anyhow::bail!("Filter expression cannot be empty");
            }
        }

        Ok(())
    }
}
