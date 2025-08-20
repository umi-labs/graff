use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "graff")]
#[command(
    about = "Fast, deterministic Rust CLI for converting CSV data into beautiful PNG/SVG/PDF charts"
)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Chart theme
    #[arg(long, global = true, default_value = "light")]
    pub theme: Theme,

    /// Canvas scale factor
    #[arg(long, global = true, default_value = "1.0")]
    pub scale: f64,

    /// Output format
    #[arg(long, global = true, default_value = "png")]
    pub format: OutputFormat,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate line charts for time series data
    Line(LineArgs),
    /// Generate area charts for composition analysis
    Area(AreaArgs),
    /// Generate bar charts for categorical comparisons
    Bar(BarArgs),
    /// Generate stacked bar charts for composition analysis
    BarStacked(BarStackedArgs),
    /// Generate heatmaps for 2D data visualization
    Heatmap(HeatmapArgs),
    /// Generate scatter plots for correlation analysis
    Scatter(ScatterArgs),
    /// Generate funnel charts for conversion analysis
    Funnel(FunnelArgs),
    /// Generate retention matrix for cohort analysis
    Retention(RetentionArgs),
    /// Batch render multiple charts from specification file
    Render(RenderArgs),
}

#[derive(Parser)]
pub struct LineArgs {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// X-axis column name
    #[arg(short, long)]
    pub x: String,

    /// Y-axis column name
    #[arg(short, long)]
    pub y: String,

    /// Group by column (creates multiple series)
    #[arg(short, long)]
    pub group: Option<String>,

    /// Aggregation function
    #[arg(short, long, default_value = "sum")]
    pub agg: AggregationType,

    /// Filter expression
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Chart title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Canvas width in pixels
    #[arg(long, default_value = "1400")]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value = "800")]
    pub height: u32,
}

#[derive(Parser)]
pub struct AreaArgs {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// X-axis column name
    #[arg(short, long)]
    pub x: String,

    /// Y-axis column name
    #[arg(short, long)]
    pub y: String,

    /// Group by column
    #[arg(short, long)]
    pub group: Option<String>,

    /// Aggregation function
    #[arg(short, long, default_value = "sum")]
    pub agg: AggregationType,

    /// Create stacked area chart
    #[arg(long, default_value = "true")]
    pub stacked: bool,

    /// Normalize to 100% for percentage view
    #[arg(long)]
    pub normalize: bool,

    /// Filter expression
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Chart title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Canvas width in pixels
    #[arg(long, default_value = "1400")]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value = "800")]
    pub height: u32,
}

#[derive(Parser)]
pub struct BarArgs {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// X-axis column name
    #[arg(short, long)]
    pub x: String,

    /// Y-axis column name
    #[arg(short, long)]
    pub y: String,

    /// Group by column
    #[arg(short, long)]
    pub group: Option<String>,

    /// Aggregation function
    #[arg(short, long, default_value = "sum")]
    pub agg: AggregationType,

    /// Create stacked bars instead of grouped
    #[arg(long)]
    pub stacked: bool,

    /// Horizontal bar chart orientation
    #[arg(long)]
    pub horizontal: bool,

    /// Filter expression
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Chart title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Canvas width in pixels
    #[arg(long, default_value = "1400")]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value = "800")]
    pub height: u32,
}

#[derive(Parser)]
pub struct HeatmapArgs {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// X-axis column name
    #[arg(short, long)]
    pub x: String,

    /// Y-axis column name
    #[arg(short, long)]
    pub y: String,

    /// Value column name (for color intensity)
    #[arg(short, long)]
    pub z: String,

    /// Number of color bins
    #[arg(long, default_value = "10")]
    pub bins: u32,

    /// Color map
    #[arg(long, default_value = "viridis")]
    pub colormap: ColorMap,

    /// Chart title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Canvas width in pixels
    #[arg(long, default_value = "1400")]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value = "800")]
    pub height: u32,
}

#[derive(Parser)]
pub struct FunnelArgs {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Comma-separated step names in order
    #[arg(short, long)]
    pub steps: String,

    /// Step order (comma-separated indices, e.g., "0,1,2,3")
    #[arg(long)]
    pub step_order: Option<String>,

    /// Value label position (left or right)
    #[arg(long, default_value = "right")]
    pub value_labels: crate::spec::ValueLabelPosition,

    /// Value column name
    #[arg(long)]
    pub values: String,

    /// Show conversion rates between steps
    #[arg(long)]
    pub conversion_rates: bool,

    /// Chart title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Canvas width in pixels
    #[arg(long, default_value = "1400")]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value = "800")]
    pub height: u32,
}

#[derive(Parser)]
pub struct RetentionArgs {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Cohort start date column
    #[arg(long)]
    pub cohort_date: String,

    /// Period number column (0, 1, 2, ...)
    #[arg(long)]
    pub period_number: String,

    /// Active users column
    #[arg(short, long)]
    pub users: String,

    /// Show retention as percentages
    #[arg(long)]
    pub percentage: bool,

    /// Chart title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Canvas width in pixels
    #[arg(long, default_value = "1400")]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value = "800")]
    pub height: u32,
}

#[derive(Parser)]
pub struct RenderArgs {
    /// YAML or JSON specification file
    #[arg(short, long)]
    pub spec: PathBuf,

    /// Override default data file from spec
    #[arg(short, long)]
    pub data: Option<PathBuf>,

    /// Output directory (defaults to ~/Desktop/graff if not specified)
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Number of parallel renders
    #[arg(short, long)]
    pub parallel: Option<usize>,
}

#[derive(Parser)]
pub struct BarStackedArgs {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// X-axis column name
    #[arg(short, long)]
    pub x: String,

    /// Y-axis column name
    #[arg(short, long)]
    pub y: String,

    /// Group by column
    #[arg(short, long)]
    pub group: Option<String>,

    /// Aggregation function
    #[arg(short, long, default_value = "sum")]
    pub agg: AggregationType,

    /// Filter expression
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Chart title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Canvas width in pixels
    #[arg(long, default_value = "1400")]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value = "800")]
    pub height: u32,
}

#[derive(Parser)]
pub struct ScatterArgs {
    /// Input CSV file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// X-axis column name
    #[arg(short, long)]
    pub x: String,

    /// Y-axis column name
    #[arg(short, long)]
    pub y: String,

    /// Group by column (for color coding)
    #[arg(short, long)]
    pub group: Option<String>,

    /// Filter expression
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Chart title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Output file path
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Canvas width in pixels
    #[arg(long, default_value = "1400")]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value = "800")]
    pub height: u32,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ValueLabelPosition {
    Left,
    Right,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Png,
    Svg,
    Pdf,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AggregationType {
    Sum,
    Count,
    Mean,
    Median,
    Min,
    Max,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorMap {
    Viridis,
    Plasma,
    Blues,
    Reds,
    Greens,
}

// Conversion functions for CLI types to spec types
fn convert_agg_type(cli_agg: &AggregationType) -> crate::spec::AggregationType {
    match cli_agg {
        AggregationType::Sum => crate::spec::AggregationType::Sum,
        AggregationType::Count => crate::spec::AggregationType::Count,
        AggregationType::Mean => crate::spec::AggregationType::Mean,
        AggregationType::Median => crate::spec::AggregationType::Median,
        AggregationType::Min => crate::spec::AggregationType::Min,
        AggregationType::Max => crate::spec::AggregationType::Max,
    }
}

fn convert_colormap_type(cli_colormap: &ColorMap) -> crate::spec::ColorMap {
    match cli_colormap {
        ColorMap::Viridis => crate::spec::ColorMap::Viridis,
        ColorMap::Plasma => crate::spec::ColorMap::Plasma,
        ColorMap::Blues => crate::spec::ColorMap::Blues,
        ColorMap::Reds => crate::spec::ColorMap::Reds,
        ColorMap::Greens => crate::spec::ColorMap::Greens,
    }
}

fn convert_theme_type(cli_theme: &Theme) -> crate::spec::Theme {
    match cli_theme {
        Theme::Light => crate::spec::Theme::Light,
        Theme::Dark => crate::spec::Theme::Dark,
    }
}

fn parse_filter_string(filter_str: &str) -> Result<crate::spec::FilterConfig> {
    // Simple filter parsing - for now just create a basic filter
    // This could be enhanced to parse more complex filter expressions
    let mut include = std::collections::HashMap::new();
    include.insert(
        "expression".to_string(),
        crate::spec::FilterValue::Single(filter_str.to_string()),
    );

    Ok(crate::spec::FilterConfig {
        include: Some(include),
        exclude: None,
        expression: Some(filter_str.to_string()),
    })
}

pub fn run(cli: Cli) -> Result<()> {
    // Set up logging based on verbosity
    if cli.verbose {
        println!("Verbose mode enabled");
    }

    match cli.command {
        Commands::Line(args) => render_line_chart_cli(args, &cli.theme),
        Commands::Area(args) => render_area_chart_cli(args, &cli.theme),
        Commands::Bar(args) => render_bar_chart_cli(args, &cli.theme),
        Commands::BarStacked(args) => render_bar_stacked_chart_cli(args, &cli.theme),
        Commands::Heatmap(args) => render_heatmap_chart_cli(args, &cli.theme),
        Commands::Scatter(args) => render_scatter_chart_cli(args, &cli.theme),
        Commands::Funnel(args) => render_funnel_chart_cli(args, &cli.theme),
        Commands::Retention(args) => render_retention_chart_cli(args, &cli.theme),
        Commands::Render(args) => render_batch_charts(args),
    }
}

fn render_line_chart_cli(args: LineArgs, theme: &Theme) -> Result<()> {
    // Create chart configuration
    let chart_config = crate::spec::ChartConfig {
        chart_type: crate::spec::ChartType::Line,
        title: args.title,
        data: Some(args.input.clone()),
        x: Some(args.x.clone()),
        y: Some(args.y.clone()),
        z: None,
        group_by: args.group.clone(),
        agg: Some(convert_agg_type(&args.agg)),
        filter: args
            .filter
            .as_ref()
            .map(|f| parse_filter_string(f))
            .transpose()?,
        derive: None,
        sort: None,
        limit: None,
        width: Some(args.width),
        height: Some(args.height),
        theme: Some(convert_theme_type(theme)),
        format: Some(crate::spec::OutputFormat::Png),
        scale: None,
        stacked: None,
        horizontal: None,
        normalize: None,
        bins: None,
        colormap: None,
        steps: None,
        step_order: None,
        value_labels: None,
        values: None,
        conversion_rates: None,
        cohort_date: None,
        period_number: None,
        users: None,
        percentage: None,
        legend_position: None,
    };

    // Determine output path
    let output_path = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        let input_stem = args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("line");
        PathBuf::from(format!("{}-line.png", input_stem))
    };

    // Render the chart using the existing pipeline
    process_single_chart(&args.input, &chart_config, &output_path)?;

    println!("✅ Generated line chart: {}", output_path.display());
    Ok(())
}

fn render_area_chart_cli(args: AreaArgs, theme: &Theme) -> Result<()> {
    // Create chart configuration
    let chart_config = crate::spec::ChartConfig {
        chart_type: crate::spec::ChartType::Area,
        title: args.title,
        data: Some(args.input.clone()),
        x: Some(args.x.clone()),
        y: Some(args.y.clone()),
        z: None,
        group_by: args.group.clone(),
        agg: Some(convert_agg_type(&args.agg)),
        filter: args
            .filter
            .as_ref()
            .map(|f| parse_filter_string(f))
            .transpose()?,
        derive: None,
        sort: None,
        limit: None,
        width: Some(args.width),
        height: Some(args.height),
        theme: Some(convert_theme_type(theme)),
        format: Some(crate::spec::OutputFormat::Png),
        scale: None,
        stacked: Some(args.stacked),
        horizontal: None,
        normalize: Some(args.normalize),
        bins: None,
        colormap: None,
        steps: None,
        step_order: None,
        value_labels: None,
        values: None,
        conversion_rates: None,
        cohort_date: None,
        period_number: None,
        users: None,
        percentage: None,
        legend_position: None,
    };

    // Determine output path
    let output_path = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        let input_stem = args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("area");
        PathBuf::from(format!("{}-area.png", input_stem))
    };

    // Render the chart using the existing pipeline
    process_single_chart(&args.input, &chart_config, &output_path)?;

    println!("✅ Generated area chart: {}", output_path.display());
    Ok(())
}

fn render_bar_chart_cli(args: BarArgs, theme: &Theme) -> Result<()> {
    // Create chart configuration
    let chart_config = crate::spec::ChartConfig {
        chart_type: crate::spec::ChartType::Bar,
        title: args.title,
        data: Some(args.input.clone()),
        x: Some(args.x.clone()),
        y: Some(args.y.clone()),
        z: None,
        group_by: args.group.clone(),
        agg: Some(convert_agg_type(&args.agg)),
        filter: args
            .filter
            .as_ref()
            .map(|f| parse_filter_string(f))
            .transpose()?,
        derive: None,
        sort: None,
        limit: None,
        width: Some(args.width),
        height: Some(args.height),
        theme: Some(convert_theme_type(theme)),
        format: Some(crate::spec::OutputFormat::Png),
        scale: None,
        stacked: Some(args.stacked),
        horizontal: Some(args.horizontal),
        normalize: None,
        bins: None,
        colormap: None,
        steps: None,
        step_order: None,
        value_labels: None,
        values: None,
        conversion_rates: None,
        cohort_date: None,
        period_number: None,
        users: None,
        percentage: None,
        legend_position: None,
    };

    // Determine output path
    let output_path = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        let input_stem = args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("bar");
        PathBuf::from(format!("{}-bar.png", input_stem))
    };

    // Render the chart using the existing pipeline
    process_single_chart(&args.input, &chart_config, &output_path)?;

    println!("✅ Generated bar chart: {}", output_path.display());
    Ok(())
}

fn render_heatmap_chart_cli(args: HeatmapArgs, theme: &Theme) -> Result<()> {
    // Create chart configuration
    let chart_config = crate::spec::ChartConfig {
        chart_type: crate::spec::ChartType::Heatmap,
        title: args.title,
        data: Some(args.input.clone()),
        x: Some(args.x.clone()),
        y: Some(args.y.clone()),
        z: Some(args.z.clone()),
        group_by: None,
        agg: None,
        filter: None,
        derive: None,
        sort: None,
        limit: None,
        width: Some(args.width),
        height: Some(args.height),
        theme: Some(convert_theme_type(theme)),
        format: Some(crate::spec::OutputFormat::Png),
        scale: None,
        stacked: None,
        horizontal: None,
        normalize: None,
        bins: Some(args.bins),
        colormap: Some(convert_colormap_type(&args.colormap)),
        steps: None,
        step_order: None,
        value_labels: None,
        values: None,
        conversion_rates: None,
        cohort_date: None,
        period_number: None,
        users: None,
        percentage: None,
        legend_position: None,
    };

    // Determine output path
    let output_path = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        let input_stem = args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("heatmap");
        PathBuf::from(format!("{}-heatmap.png", input_stem))
    };

    // Render the chart using the existing pipeline
    process_single_chart(&args.input, &chart_config, &output_path)?;

    println!("✅ Generated heatmap: {}", output_path.display());
    Ok(())
}

fn render_retention_chart_cli(args: RetentionArgs, theme: &Theme) -> Result<()> {
    // Create chart configuration
    let chart_config = crate::spec::ChartConfig {
        chart_type: crate::spec::ChartType::Retention,
        title: args.title,
        data: Some(args.input.clone()),
        x: None,
        y: None,
        z: None,
        group_by: None,
        agg: None,
        filter: None,
        derive: None,
        sort: None,
        limit: None,
        width: Some(args.width),
        height: Some(args.height),
        theme: Some(convert_theme_type(theme)),
        format: Some(crate::spec::OutputFormat::Png),
        scale: None,
        stacked: None,
        horizontal: None,
        normalize: None,
        bins: None,
        colormap: None,
        steps: None,
        step_order: None,
        value_labels: None,
        values: None,
        conversion_rates: None,
        cohort_date: Some(args.cohort_date.clone()),
        period_number: Some(args.period_number.clone()),
        users: Some(args.users.clone()),
        percentage: Some(args.percentage),
        legend_position: None,
    };

    // Determine output path
    let output_path = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        let input_stem = args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("retention");
        PathBuf::from(format!("{}-retention.png", input_stem))
    };

    // Render the chart using the existing pipeline
    process_single_chart(&args.input, &chart_config, &output_path)?;

    println!("✅ Generated retention chart: {}", output_path.display());
    Ok(())
}

fn render_bar_stacked_chart_cli(args: BarStackedArgs, theme: &Theme) -> Result<()> {
    // Create chart configuration
    let chart_config = crate::spec::ChartConfig {
        chart_type: crate::spec::ChartType::BarStacked,
        title: args.title,
        data: Some(args.input.clone()),
        x: Some(args.x.clone()),
        y: Some(args.y.clone()),
        z: None,
        group_by: args.group.clone(),
        agg: Some(convert_agg_type(&args.agg)),
        filter: args
            .filter
            .as_ref()
            .map(|f| parse_filter_string(f))
            .transpose()?,
        derive: None,
        sort: None,
        limit: None,
        width: Some(args.width),
        height: Some(args.height),
        theme: Some(convert_theme_type(theme)),
        format: Some(crate::spec::OutputFormat::Png),
        scale: None,
        stacked: Some(true), // Always true for stacked bars
        horizontal: None,
        normalize: None,
        bins: None,
        colormap: None,
        steps: None,
        step_order: None,
        value_labels: None,
        values: None,
        conversion_rates: None,
        cohort_date: None,
        period_number: None,
        users: None,
        percentage: None,
        legend_position: None,
    };

    // Determine output path
    let output_path = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        let input_stem = args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("bar-stacked");
        PathBuf::from(format!("{}-bar-stacked.png", input_stem))
    };

    // Render the chart using the existing pipeline
    process_single_chart(&args.input, &chart_config, &output_path)?;

    println!("✅ Generated stacked bar chart: {}", output_path.display());
    Ok(())
}

fn render_scatter_chart_cli(args: ScatterArgs, theme: &Theme) -> Result<()> {
    // Create chart configuration
    let chart_config = crate::spec::ChartConfig {
        chart_type: crate::spec::ChartType::Scatter,
        title: args.title,
        data: Some(args.input.clone()),
        x: Some(args.x.clone()),
        y: Some(args.y.clone()),
        z: None,
        group_by: args.group.clone(),
        agg: None, // No aggregation for scatter plots
        filter: args
            .filter
            .as_ref()
            .map(|f| parse_filter_string(f))
            .transpose()?,
        derive: None,
        sort: None,
        limit: None,
        width: Some(args.width),
        height: Some(args.height),
        theme: Some(convert_theme_type(theme)),
        format: Some(crate::spec::OutputFormat::Png),
        scale: None,
        stacked: None,
        horizontal: None,
        normalize: None,
        bins: None,
        colormap: None,
        steps: None,
        step_order: None,
        value_labels: None,
        values: None,
        conversion_rates: None,
        cohort_date: None,
        period_number: None,
        users: None,
        percentage: None,
        legend_position: None,
    };

    // Determine output path
    let output_path = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        let input_stem = args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("scatter");
        PathBuf::from(format!("{}-scatter.png", input_stem))
    };

    // Render the chart using the existing pipeline
    process_single_chart(&args.input, &chart_config, &output_path)?;

    println!("✅ Generated scatter plot: {}", output_path.display());
    Ok(())
}

fn render_funnel_chart_cli(args: FunnelArgs, theme: &Theme) -> Result<()> {
    // Parse steps from comma-separated string
    let steps: Vec<String> = args
        .steps
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if steps.is_empty() {
        anyhow::bail!("No steps provided");
    }

    // Handle step ordering (interactive or from args)
    let step_order = handle_funnel_step_ordering(&steps, &args.step_order)?;

    // Create chart configuration
    let chart_config = crate::spec::ChartConfig {
        chart_type: crate::spec::ChartType::Funnel,
        title: args.title,
        data: Some(args.input.clone()),
        x: None,
        y: None,
        z: None,
        group_by: None,
        agg: None,
        filter: None,
        derive: None,
        sort: None,
        limit: None,
        width: Some(args.width),
        height: Some(args.height),
        theme: Some(convert_theme_type(theme)),
        format: Some(crate::spec::OutputFormat::Png),
        scale: None,
        stacked: None,
        horizontal: None,
        normalize: None,
        bins: None,
        colormap: None,
        steps: Some(steps),
        step_order: Some(step_order),
        value_labels: Some(args.value_labels),
        values: Some(args.values),
        conversion_rates: None,
        cohort_date: None,
        period_number: None,
        users: None,
        percentage: None,
        legend_position: None,
    };

    // Determine output path
    let output_path = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        let input_stem = args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("funnel");
        PathBuf::from(format!("{}-funnel.png", input_stem))
    };

    // Render the chart using the existing pipeline
    process_single_chart(&args.input, &chart_config, &output_path)?;

    println!("✅ Generated funnel chart: {}", output_path.display());
    Ok(())
}

fn handle_funnel_step_ordering(
    steps: &[String],
    step_order_arg: &Option<String>,
) -> Result<Vec<usize>> {
    if let Some(step_order_str) = step_order_arg {
        // Parse provided step order
        let order: Result<Vec<usize>, _> = step_order_str
            .split(',')
            .map(|s| s.trim().parse::<usize>())
            .collect();
        let order = order.map_err(|e| anyhow::anyhow!("Invalid step order: {}", e))?;

        // Validate step order
        validate_step_order(&order, steps.len())?;
        println!("✅ Using step order: {:?}", order);
        Ok(order)
    } else {
        // Interactive step ordering
        println!("\n🎯 Funnel Step Ordering");
        println!("Available steps:");
        for (i, step) in steps.iter().enumerate() {
            println!("  {}: {}", i, step);
        }

        println!("\nDefault order (by value): [0, 1, 2, 3, ...]");
        println!("Enter custom order (comma-separated indices) or press Enter for default:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let order = if input.is_empty() {
            // Use default order (0, 1, 2, 3, ...)
            (0..steps.len()).collect()
        } else {
            // Parse custom order
            let order: Result<Vec<usize>, _> = input
                .split(',')
                .map(|s| s.trim().parse::<usize>())
                .collect();
            order.map_err(|e| anyhow::anyhow!("Invalid step order: {}", e))?
        };

        // Validate step order
        validate_step_order(&order, steps.len())?;
        println!("✅ Using step order: {:?}", order);
        Ok(order)
    }
}

fn validate_step_order(step_order: &[usize], num_steps: usize) -> Result<()> {
    if step_order.len() != num_steps {
        anyhow::bail!(
            "Step order length ({}) must match number of steps ({})",
            step_order.len(),
            num_steps
        );
    }

    for &idx in step_order {
        if idx >= num_steps {
            anyhow::bail!("Invalid step order index: {} (max: {})", idx, num_steps - 1);
        }
    }

    Ok(())
}

fn render_batch_charts(args: RenderArgs) -> Result<()> {
    println!("Loading spec file: {}", args.spec.display());

    // Read and parse the spec file
    let spec_content = fs::read_to_string(&args.spec).map_err(|e| {
        anyhow::anyhow!("Failed to read spec file '{}': {}", args.spec.display(), e)
    })?;

    let spec = if args.spec.extension().and_then(|s| s.to_str()) == Some("json") {
        crate::spec::ChartSpec::from_json(&spec_content)?
    } else {
        crate::spec::ChartSpec::from_yaml(&spec_content)?
    };

    println!("Parsed spec with {} charts", spec.charts.len());

    // Use user-specified output directory, or default to ~/Desktop/graff
    let output_dir = if let Some(out_path) = &args.out {
        out_path.clone()
    } else {
        // Check if we're in development mode (running from the graff repo)
        if std::env::current_dir()
            .unwrap_or_default()
            .ends_with("graff")
        {
            // Development mode: use tests/output for easier testing
            PathBuf::from("tests/output")
        } else {
            // Production mode: default to ~/Desktop/graff when no output specified
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join("Desktop").join("graff")
        }
    };

    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
        println!("Created output directory: {}", output_dir.display());
    }

    // Process each chart
    let mut successful_charts = 0;
    let mut failed_charts = 0;

    for (index, chart_config) in spec.charts.iter().enumerate() {
        let default_name = format!("chart_{}", index + 1);
        let chart_name = chart_config.title.as_deref().unwrap_or(&default_name);

        println!(
            "Processing chart {}: {} ({:?})",
            index + 1,
            chart_name,
            chart_config.chart_type
        );

        // Determine data source
        let data_path = chart_config
            .data
            .as_ref()
            .or(spec.data.as_ref().and_then(|d| d.default.as_ref()))
            .ok_or_else(|| {
                anyhow::anyhow!("No data source specified for chart '{}'", chart_name)
            })?;

        println!("  Data source: {}", data_path.display());

        // Generate output filename
        let output_format = chart_config
            .format
            .as_ref()
            .unwrap_or(&crate::spec::OutputFormat::Png);
        let extension = match output_format {
            crate::spec::OutputFormat::Png => "png",
            crate::spec::OutputFormat::Svg => "svg",
            crate::spec::OutputFormat::Pdf => "pdf",
        };

        let filename = format!(
            "{}-{:?}.{}",
            chart_name.to_lowercase().replace(' ', "-"),
            chart_config.chart_type,
            extension
        );
        let output_path = output_dir.join(filename);

        // For now, just log what we would do
        // TODO: Implement actual chart rendering
        match process_single_chart(data_path, chart_config, &output_path) {
            Ok(()) => {
                successful_charts += 1;
                println!("✓ Generated: {}", output_path.display());
            }
            Err(e) => {
                failed_charts += 1;
                eprintln!("✗ Failed to generate '{}': {:?}", chart_name, e);
            }
        }
    }

    // Print summary
    println!(
        "\nSummary: {} successful, {} failed",
        successful_charts, failed_charts
    );

    if failed_charts > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn process_single_chart(
    data_path: &Path,
    chart_config: &crate::spec::ChartConfig,
    output_path: &Path,
) -> Result<()> {
    // Validate the chart config
    chart_config.validate()?;

    // Load CSV data
    let load_options = crate::data::LoadOptions::default();
    let lf = crate::data::load_csv(data_path, &load_options)
        .with_context(|| format!("Failed to load data from {}", data_path.display()))?;

    // Validate required columns exist
    let required_columns = get_required_columns(chart_config);
    crate::data::validate_columns(&lf, &required_columns).with_context(|| {
        format!(
            "Column validation failed for chart '{}'",
            chart_config.title.as_deref().unwrap_or("unnamed")
        )
    })?;

    // Get column info for reporting
    let available_columns = crate::data::get_column_names(&lf)?;
    println!(
        "  Loaded data with {} columns: {:?}",
        available_columns.len(),
        available_columns
    );

    // Apply transformations (filters, grouping, aggregation)
    let processed_lf = apply_chart_transformations(lf, chart_config)?;

    // Render chart with Plotters
    crate::render::render_chart(processed_lf, chart_config, output_path)
        .with_context(|| format!("Failed to render chart to {}", output_path.display()))?;

    Ok(())
}

fn apply_chart_transformations(
    mut lf: polars::prelude::LazyFrame,
    config: &crate::spec::ChartConfig,
) -> Result<polars::prelude::LazyFrame> {
    // Apply filters if specified
    if let Some(filter) = &config.filter {
        lf = apply_filter_config(lf, filter)?;
    }

    // Apply grouping and aggregation if specified
    if let Some(agg) = &config.agg {
        // For charts with aggregation, group by the x-axis column unless explicitly specified
        let group_by_col = config
            .group_by
            .as_ref()
            .unwrap_or(config.x.as_ref().unwrap());
        lf = apply_aggregation(lf, group_by_col, config.y.as_ref().unwrap(), agg)?;
    } else if let Some(_group_by) = &config.group_by {
        // Handle grouping without aggregation (for line charts, etc.)
        // For now, just pass through - we might want to implement grouping logic here
    }

    // Apply sorting if specified
    if let Some(sort) = &config.sort {
        for sort_config in sort {
            let ascending = sort_config.ascending.unwrap_or(true);
            let options = polars::prelude::SortOptions {
                descending: !ascending,
                ..Default::default()
            };
            lf = lf.sort(&sort_config.column, options);
        }
    }

    // Apply limit if specified
    if let Some(limit) = config.limit {
        lf = lf.limit(limit as u32);
    }

    Ok(lf)
}

fn apply_filter_config(
    mut lf: polars::prelude::LazyFrame,
    filter: &crate::spec::FilterConfig,
) -> Result<polars::prelude::LazyFrame> {
    use polars::prelude::*;

    // Apply include filters
    if let Some(includes) = &filter.include {
        for (column, values) in includes {
            let filter_expr = match values {
                crate::spec::FilterValue::Single(value) => col(column).eq(lit(value.clone())),
                crate::spec::FilterValue::Multiple(values) => {
                    let mut expr = col(column).eq(lit(values[0].clone()));
                    for value in values.iter().skip(1) {
                        expr = expr.or(col(column).eq(lit(value.clone())));
                    }
                    expr
                }
            };
            lf = lf.filter(filter_expr);
        }
    }

    // Apply exclude filters
    if let Some(excludes) = &filter.exclude {
        for (column, values) in excludes {
            let filter_expr = match values {
                crate::spec::FilterValue::Single(value) => col(column).neq(lit(value.clone())),
                crate::spec::FilterValue::Multiple(values) => {
                    let mut expr = col(column).neq(lit(values[0].clone()));
                    for value in values.iter().skip(1) {
                        expr = expr.and(col(column).neq(lit(value.clone())));
                    }
                    expr
                }
            };
            lf = lf.filter(filter_expr);
        }
    }

    Ok(lf)
}

fn apply_aggregation(
    lf: polars::prelude::LazyFrame,
    group_by: &str,
    value_col: &str,
    agg_type: &crate::spec::AggregationType,
) -> Result<polars::prelude::LazyFrame> {
    use polars::prelude::*;

    let agg_expr = match agg_type {
        crate::spec::AggregationType::Sum => col(value_col).sum(),
        crate::spec::AggregationType::Mean => col(value_col).mean(),
        crate::spec::AggregationType::Count => col(value_col).count(),
        crate::spec::AggregationType::Min => col(value_col).min(),
        crate::spec::AggregationType::Max => col(value_col).max(),
        crate::spec::AggregationType::Median => col(value_col).median(),
    };

    Ok(lf
        .group_by([col(group_by)])
        .agg([agg_expr.alias(value_col)]))
}

fn get_required_columns(chart_config: &crate::spec::ChartConfig) -> Vec<String> {
    let mut columns = Vec::new();

    // Add x and y columns if they exist (for charts that need them)
    if let Some(x) = &chart_config.x {
        columns.push(x.clone());
    }
    if let Some(y) = &chart_config.y {
        columns.push(y.clone());
    }

    // Add chart-type specific required columns
    match chart_config.chart_type {
        crate::spec::ChartType::Heatmap => {
            if let Some(z) = &chart_config.z {
                columns.push(z.clone());
            }
        }
        crate::spec::ChartType::Retention => {
            if let Some(cohort_date) = &chart_config.cohort_date {
                columns.push(cohort_date.clone());
            }
            if let Some(period_number) = &chart_config.period_number {
                columns.push(period_number.clone());
            }
            if let Some(users) = &chart_config.users {
                columns.push(users.clone());
            }
        }
        _ => {}
    }

    // Add optional columns if they exist
    if let Some(group_by) = &chart_config.group_by {
        columns.push(group_by.clone());
    }

    columns
}
