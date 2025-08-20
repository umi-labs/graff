use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChartType {
    Line,
    Area,
    Bar,
    BarStacked,
    Heatmap,
    Funnel,
    Retention,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregationType {
    Sum,
    Count,
    Mean,
    Median,
    Min,
    Max,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Png,
    Svg,
    Pdf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorMap {
    Viridis,
    Plasma,
    Blues,
    Reds,
    Greens,
}
