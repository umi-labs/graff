# Architecture

This document describes the system design and data flow architecture of Graff.

## Overview

Graff follows a modular, pipeline-based architecture that separates concerns between data loading, transformation, chart rendering, and output generation. The design prioritizes performance, memory efficiency, and extensibility.

```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐    ┌──────────────┐
│   CLI Args  │───▶│ Data Loader  │───▶│ Transform   │───▶│  Renderer    │
│   or Spec   │    │   (Polars)   │    │  Pipeline   │    │  (Plotters)  │
└─────────────┘    └──────────────┘    └─────────────┘    └──────────────┘
                            │                   │                   │
                            ▼                   ▼                   ▼
                   ┌──────────────┐    ┌─────────────┐    ┌──────────────┐
                   │ Schema Auto- │    │ Derived     │    │ PNG/SVG/PDF  │
                   │ Detection    │    │ Columns     │    │ Output       │
                   └──────────────┘    └─────────────┘    └──────────────┘
```

## Core Components

### 1. CLI Layer (`src/cli.rs`)

**Responsibility**: Parse command-line arguments and dispatch to appropriate handlers.

**Key Features**:
- Uses `clap` v4 with subcommands for each chart type
- Validates required arguments and provides helpful error messages
- Supports both direct CLI usage and spec file processing
- Global options (theme, scale, format) applied to all charts

**Command Structure**:
```rust
enum Command {
    Line(LineArgs),
    Area(AreaArgs), 
    Bar(BarArgs),
    Heatmap(HeatmapArgs),
    Funnel(FunnelArgs),
    Retention(RetentionArgs),
    Render(RenderArgs),
}
```

### 2. Spec Parser (`src/spec.rs`)

**Responsibility**: Parse and validate YAML/JSON specification files.

**Key Features**:
- Deserializes spec files using `serde_yaml` and `serde_json`
- Validates chart configurations against required fields per chart type
- Supports data source overrides per chart
- Provides detailed validation errors with field paths

**Spec Structure**:
```rust
#[derive(Deserialize)]
struct ChartSpec {
    data: DataConfig,
    charts: Vec<ChartConfig>,
}

#[derive(Deserialize)]
struct ChartConfig {
    chart_type: ChartType,
    title: Option<String>,
    x: String,
    y: String,
    group_by: Option<String>,
    filter: Option<FilterConfig>,
    derive: Option<HashMap<String, String>>,
    agg: Option<AggregationType>,
    // ... chart-specific fields
}
```

### 3. Data Layer (`src/data/`)

#### Data Loader (`loader.rs`)
**Responsibility**: Load CSV files into Polars LazyFrames with schema detection.

**Key Features**:
- Auto-detects GA4 date formats (YYYYMMDD vs ISO)
- Infers schema with fallback to string types
- Supports streaming for large files
- Column name validation with suggestions for typos

**Loading Pipeline**:
```rust
pub fn load_csv(path: &Path, options: &LoadOptions) -> Result<LazyFrame> {
    LazyFrame::scan_csv(path, scan_args)
        .with_auto_schema_detection()
        .with_ga4_date_parsing()
        .with_column_validation()
}
```

#### Derived Columns (`derive.rs`)
**Responsibility**: Implement common GA4/BigQuery transformations.

**Available Functions**:
- `to_week(date)` - Convert to Monday week start
- `to_month(date)` - Convert to first of month  
- `to_hour(timestamp)` - Extract hour (0-23)
- `weekday(date)` - Day of week (Mon-Sun)
- `source_medium(source, medium)` - Combine traffic source

**Implementation**:
```rust
pub fn apply_derived_columns(
    lf: LazyFrame, 
    derivations: &HashMap<String, String>
) -> Result<LazyFrame> {
    derivations.iter().try_fold(lf, |acc, (col_name, expr)| {
        let derived_expr = parse_derive_expression(expr)?;
        Ok(acc.with_columns([derived_expr.alias(col_name)]))
    })
}
```

#### Transform Pipeline (`transform.rs`)
**Responsibility**: Apply filters, grouping, aggregation, and sorting.

**Pipeline Stages**:
1. **Filter**: Apply include/exclude conditions
2. **Derive**: Add computed columns
3. **Group**: Group by specified columns
4. **Aggregate**: Apply aggregation functions (sum, count, mean, etc.)
5. **Sort**: Order results for consistent output
6. **Limit**: Apply row limits if specified

**Transform Flow**:
```rust
pub fn apply_transforms(
    lf: LazyFrame,
    config: &TransformConfig,
) -> Result<LazyFrame> {
    lf.pipe(|df| apply_filters(df, &config.filter))?
      .pipe(|df| apply_derived_columns(df, &config.derive))?
      .pipe(|df| apply_grouping(df, &config.group_by, &config.agg))?
      .pipe(|df| apply_sorting(df, &config.sort))?
      .pipe(|df| apply_limit(df, config.limit))
}
```

### 4. Chart Layer (`src/chart/`)

#### Chart Types (`types.rs`)
**Responsibility**: Define chart type enums and configuration structs.

**Core Types**:
```rust
#[derive(Debug, Clone, Deserialize)]
pub enum ChartType {
    Line,
    Area, 
    Bar,
    BarStacked,
    Heatmap,
    Funnel,
    Retention,
}

#[derive(Debug, Clone, Deserialize)]
pub enum AggregationType {
    Sum,
    Count,
    Mean,
    Median,
    Min,
    Max,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}
```

#### Chart Implementations
Each chart type has its own module with specialized rendering logic:

- **Line Charts** (`line.rs`): Time series with multiple series support
- **Area Charts** (`area.rs`): Stacked area plots for composition analysis
- **Bar Charts** (`bar.rs`): Grouped and stacked bars with horizontal option
- **Heatmaps** (`heatmap.rs`): 2D discrete/continuous value mapping
- **Funnels** (`funnel.rs`): Sequential step conversion analysis
- **Retention** (`retention.rs`): Cohort retention matrix visualization

#### Color Palette (`palette.rs`)
**Responsibility**: Provide colorblind-friendly color schemes.

**Features**:
- Light and dark theme variants
- Colorblind-safe palette (8+ colors)
- Automatic color assignment for series
- Custom color override support

### 5. Render Layer (`src/render/`)

#### Renderer Trait (`mod.rs`)
**Responsibility**: Define common interface for all chart renderers.

```rust
pub trait ChartRenderer {
    fn render(
        &self,
        data: LazyFrame,
        spec: &ChartSpec,
        output_path: &Path,
    ) -> Result<()>;
    
    fn validate_spec(&self, spec: &ChartSpec) -> Result<()>;
    fn required_columns(&self) -> Vec<&'static str>;
}
```

#### Plotters Integration
**Responsibility**: Implement rendering using the Plotters library.

**Output Formats**:
- **PNG**: Default bitmap format using `plotters-bitmap`
- **SVG**: Vector format using `plotters-svg` 
- **PDF**: Print format using `plotters-pdf`

**Canvas Management**:
```rust
pub fn create_canvas(
    path: &Path,
    format: OutputFormat,
    width: u32,
    height: u32,
    scale: f64,
) -> Result<Box<dyn DrawingBackend>> {
    match format {
        OutputFormat::Png => Ok(Box::new(BitMapBackend::new(path, (width, height)))),
        OutputFormat::Svg => Ok(Box::new(SVGBackend::new(path, (width, height)))),
        OutputFormat::Pdf => Ok(Box::new(PdfBackend::new(path, (width, height)))),
    }
}
```

## Data Flow

### Single Chart Rendering
```
1. Parse CLI args → ChartConfig
2. Load CSV → LazyFrame  
3. Apply transforms → Processed LazyFrame
4. Validate chart requirements → Column checks
5. Render chart → PNG/SVG/PDF output
6. Return success/error code
```

### Batch Rendering (Spec File)
```
1. Parse spec file → ChartSpec with multiple ChartConfigs
2. Load default data → Shared LazyFrame
3. For each chart (parallel):
   a. Override data source if specified
   b. Apply chart-specific transforms
   c. Render to output directory
4. Collect results → Success count + error details
5. Return aggregated exit code
```

## Memory Management

### Lazy Evaluation
- Uses Polars LazyFrame throughout pipeline
- Transformations build query plan without execution
- `.collect()` only called at render time
- Enables query optimization and memory efficiency

### Streaming Processing
- For large files (>1GB), enables streaming mode
- Processes data in chunks to limit memory usage
- Maintains deterministic output through stable sorting

### Parallel Rendering
- Spec files render charts concurrently using `rayon`
- Each chart gets independent data processing pipeline
- Shared data source loaded once, cloned for each chart
- CPU-bound rendering parallelized across available cores

## Error Handling

### Error Types
```rust
#[derive(thiserror::Error, Debug)]
pub enum GraffError {
    #[error("Data loading failed: {0}")]
    DataError(#[from] PolarsError),
    
    #[error("Chart specification invalid: {field}")]
    SpecError { field: String },
    
    #[error("Column '{column}' not found. Available: {available:?}")]
    ColumnNotFound { column: String, available: Vec<String> },
    
    #[error("Rendering failed: {0}")]
    RenderError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### Error Context
- Provides actionable error messages with suggestions
- Includes available column names for typos
- Shows file paths and line numbers for spec errors
- Suggests valid aggregation types and chart options

## Extensibility

### Adding New Chart Types
1. Create new module in `src/chart/`
2. Implement `ChartRenderer` trait
3. Add variant to `ChartType` enum
4. Add CLI subcommand in `src/cli.rs`
5. Add tests and documentation

### Adding New Aggregation Types
1. Add variant to `AggregationType` enum
2. Implement aggregation logic in `transform.rs`
3. Add validation and tests
4. Update CLI help text

### Adding New Output Formats
1. Add variant to `OutputFormat` enum
2. Implement backend creation in `render/mod.rs`
3. Add format-specific dependencies to `Cargo.toml`
4. Update CLI options and documentation

## Testing Strategy

### Unit Tests
- Data loading and schema detection
- Transform pipeline functions
- Derived column calculations
- Error handling and validation

### Integration Tests
- End-to-end CLI command execution
- Spec file parsing and validation
- Chart rendering with fixture data
- Error message accuracy

### Snapshot Tests
- Chart output checksums using `insta`
- Metadata comparison (dimensions, colors)
- Regression testing for visual changes
- Cross-platform rendering consistency

## Performance Considerations

### Data Processing
- Lazy evaluation prevents unnecessary computation
- Column selection reduces memory usage
- Streaming mode for large datasets
- Query optimization through Polars

### Rendering
- Parallel chart generation in batch mode
- Memory-mapped output for large images
- Efficient color palette management
- Canvas reuse where possible

### File I/O
- Streaming CSV parsing
- Buffered output writing  
- Temporary file cleanup
- Error-safe file operations
