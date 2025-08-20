# API Reference

This document provides detailed reference for Graff CLI commands and specification file formats.

## CLI Commands

### Global Options

Available for all commands:

```
-v, --verbose           Enable verbose logging
-q, --quiet            Suppress all output except errors  
-h, --help             Print help information
-V, --version          Print version information
    --theme <THEME>    Chart theme [default: light] [possible values: light, dark]
    --scale <SCALE>    Canvas scale factor [default: 1.0]
    --format <FORMAT>  Output format [default: png] [possible values: png, svg, pdf]
```

### `graff line`

Generate line charts for time series data.

```bash
graff line [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN>
```

#### Required Arguments
- `--input <FILE>` - Input CSV file path
- `--x <COLUMN>` - X-axis column name  
- `--y <COLUMN>` - Y-axis column name

#### Optional Arguments
- `--group <COLUMN>` - Group by column (creates multiple series)
- `--agg <AGG>` - Aggregation function [default: sum] [possible values: sum, count, mean, median, min, max]
- `--filter <EXPR>` - Filter expression (e.g., "channel='Organic'")
- `--title <TITLE>` - Chart title
- `--out <FILE>` - Output file path [default: auto-generated]
- `--width <WIDTH>` - Canvas width in pixels [default: 1400]
- `--height <HEIGHT>` - Canvas height in pixels [default: 800]

#### Examples
```bash
# Simple line chart
graff line --input users.csv --x date --y totalUsers --out users.png

# Grouped by channel with aggregation
graff line --input users.csv --x date --y totalUsers --group channel --agg sum --title "Daily Users by Channel"

# With filtering
graff line --input users.csv --x date --y totalUsers --group channel --filter "channel IN ('Organic', 'Direct')"
```

### `graff area`

Generate area charts for composition analysis.

```bash
graff area [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN>
```

#### Arguments
Same as `line` command, with additional:
- `--stacked` - Create stacked area chart [default: true]
- `--normalize` - Normalize to 100% for percentage view

#### Examples
```bash
# Stacked area showing composition
graff area --input events.csv --x date --y eventCount --group eventName --stacked

# Normalized percentage view
graff area --input sessions.csv --x date --y sessions --group deviceCategory --normalize
```

### `graff bar`

Generate bar charts for categorical comparisons.

```bash
graff bar [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN>
```

#### Arguments
Same as `line` command, with additional:
- `--stacked` - Create stacked bars instead of grouped
- `--horizontal` - Horizontal bar chart orientation

#### Examples
```bash
# Grouped bars
graff bar --input events.csv --x eventName --y eventCount --group deviceCategory

# Stacked bars  
graff bar --input events.csv --x week --y eventCount --group eventName --stacked

# Horizontal bars
graff bar --input channels.csv --x channel --y sessions --horizontal --title "Sessions by Channel"
```

### `graff bar-stacked`

Generate stacked bar charts for composition analysis.

```bash
graff bar-stacked [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN>
```

#### Arguments
Same as `bar` command, but optimized for stacked visualization.

#### Examples
```bash
# Stacked bars for composition analysis
graff bar-stacked --input events.csv --x week --y eventCount --group eventName

# With aggregation
graff bar-stacked --input sessions.csv --x date --y sessions --group deviceCategory --agg sum
```

### `graff scatter`

Generate scatter plots for correlation analysis.

```bash
graff scatter [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN>
```

#### Arguments
Same as `line` command, with additional:
- `--group <COLUMN>` - Group by column (for color coding points)

#### Examples
```bash
# Basic scatter plot
graff scatter --input data.csv --x x_value --y y_value --title "Correlation Analysis"

# With grouping for color coding
graff scatter --input data.csv --x x_value --y y_value --group category --title "Correlation by Category"
```

### `graff heatmap`

Generate heatmaps for 2D data visualization.

```bash
graff heatmap [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN> --z <COLUMN>
```

#### Required Arguments
- `--input <FILE>` - Input CSV file path
- `--x <COLUMN>` - X-axis column name
- `--y <COLUMN>` - Y-axis column name  
- `--z <COLUMN>` - Value column name (for color intensity)

#### Optional Arguments
- `--bins <N>` - Number of color bins [default: 10]
- `--colormap <MAP>` - Color map [default: viridis] [possible values: viridis, plasma, blues, reds, greens]
- `--title <TITLE>` - Chart title
- `--out <FILE>` - Output file path

#### Examples
```bash
# Hour vs weekday sessions heatmap
graff heatmap --input hourly.csv --x hour --y weekday --z sessions --title "Session Heatmap"

# Custom color scheme
graff heatmap --input data.csv --x category --y segment --z value --colormap plasma --bins 15
```

### `graff funnel`

Generate funnel charts for conversion analysis.

```bash
graff funnel [OPTIONS] --input <FILE> --steps <STEPS> --values <COLUMN>
```

#### Required Arguments
- `--input <FILE>` - Input CSV file path
- `--steps <STEPS>` - Comma-separated step names in order
- `--values <COLUMN>` - Value column name

#### Optional Arguments
- `--conversion-rates` - Show conversion rates between steps
- `--title <TITLE>` - Chart title
- `--out <FILE>` - Output file path

#### Examples
```bash
# E-commerce funnel
graff funnel --input funnel.csv --steps "page_view,add_to_cart,checkout,purchase" --values eventCount --conversion-rates

# Custom title
graff funnel --input funnel.csv --steps "impression,click,conversion" --values count --title "Ad Funnel Performance"
```

### `graff retention`

Generate retention matrix for cohort analysis.

```bash
graff retention [OPTIONS] --input <FILE> --cohort-date <COLUMN> --period-number <COLUMN> --users <COLUMN>
```

#### Required Arguments
- `--input <FILE>` - Input CSV file path
- `--cohort-date <COLUMN>` - Cohort start date column
- `--period-number <COLUMN>` - Period number column (0, 1, 2, ...)
- `--users <COLUMN>` - Active users column

#### Optional Arguments
- `--title <TITLE>` - Chart title
- `--out <FILE>` - Output file path
- `--percentage` - Show retention as percentages

#### Examples
```bash
# Weekly retention matrix
graff retention --input retention.csv --cohort-date first_seen --period-number week_number --users active_users --percentage

# Monthly cohorts
graff retention --input monthly_retention.csv --cohort-date cohort_month --period-number month_number --users retained_users
```

### `graff render`

Batch render multiple charts from specification file.

```bash
graff render [OPTIONS] --spec <FILE>
```

#### Required Arguments
- `--spec <FILE>` - YAML or JSON specification file

#### Optional Arguments
- `--data <FILE>` - Override default data file from spec
- `--out <DIR>` - Output directory [default: ./charts]
- `--parallel <N>` - Number of parallel renders [default: CPU cores]

#### Examples
```bash
# Render all charts from spec
graff render --spec dashboard.yaml --out ./output

# Override data source
graff render --spec charts.yaml --data latest_data.csv --out ./reports

# Control parallelism
graff render --spec large_spec.yaml --parallel 4
```

## Specification File Format

Specification files use YAML or JSON format to define multiple charts in a single configuration.

### Basic Structure

```yaml
# Optional global data configuration
data:
  default: ga4_data.csv
  
# Chart definitions
charts:
  - type: line
    title: "Chart Title"
    # ... chart-specific configuration
    
  - type: bar
    # ... another chart configuration
```

### Data Configuration

```yaml
data:
  # Default data file for all charts
  default: ga4_data.csv
  
  # Named data sources that charts can reference
  sources:
    events: ga4_events.csv
    sessions: ga4_sessions.csv
    users: ga4_users.csv
```

### Chart Configuration

#### Common Fields

All chart types support these fields:

```yaml
type: line              # Required: Chart type
title: "Chart Title"    # Optional: Chart title
data: events.csv        # Optional: Override data source
width: 1400            # Optional: Canvas width [default: 1400]
height: 800            # Optional: Canvas height [default: 800]
theme: light           # Optional: Theme [default: light]
format: png            # Optional: Output format [default: png]
scale: 1.0            # Optional: Scale factor [default: 1.0]
```

#### Data Processing Fields

```yaml
# Column mappings
x: date                # Required: X-axis column
y: totalUsers         # Required: Y-axis column  
group_by: channel     # Optional: Grouping column

# Aggregation
agg: sum              # Optional: Aggregation function [default: sum]

# Filtering
filter:
  include:
    channel: ["Organic Search", "Direct"]
    deviceCategory: ["desktop", "mobile"]
  exclude:
    eventName: ["session_start"]
  expression: "totalUsers > 100"  # Custom filter expression

# Derived columns
derive:
  week_start: "to_week(date)"
  hour_of_day: "to_hour(timestamp)"
  traffic_source: "source_medium(source, medium)"

# Sorting and limiting
sort:
  - column: date
    ascending: true
  - column: totalUsers  
    ascending: false
limit: 1000           # Optional: Limit number of rows
```

### Chart Type Specific Fields

#### Line Charts
```yaml
type: line
# All common fields supported
# No additional fields
```

#### Area Charts
```yaml
type: area
stacked: true         # Optional: Stack areas [default: true]
normalize: false      # Optional: Normalize to 100% [default: false]
```

#### Bar Charts
```yaml
type: bar
stacked: false        # Optional: Stack bars [default: false] 
horizontal: false     # Optional: Horizontal orientation [default: false]
```

#### Stacked Bar Charts
```yaml
type: bar-stacked
horizontal: false     # Optional: Horizontal orientation [default: false]
```

#### Heatmaps
```yaml
type: heatmap
z: sessions          # Required: Value column for color intensity
bins: 10             # Optional: Number of color bins [default: 10]
colormap: viridis    # Optional: Color scheme [default: viridis]
```

#### Funnels
```yaml
type: funnel
steps: ["page_view", "add_to_cart", "purchase"]  # Required: Step names in order
values: eventCount                               # Required: Value column
conversion_rates: true                           # Optional: Show conversion rates [default: false]
```

#### Retention Matrix
```yaml
type: retention
cohort_date: first_seen      # Required: Cohort start date column
period_number: week_number   # Required: Period number column
users: active_users         # Required: User count column
percentage: true            # Optional: Show as percentages [default: false]
```

### Complete Example

```yaml
data:
  default: ga4_data.csv
  sources:
    events: ga4_events.csv
    sessions: hourly_sessions.csv

charts:
  # Line chart with grouping and filtering
  - type: line
    title: "Daily Users by Top Channels"
    x: date
    y: totalUsers
    group_by: channel
    agg: sum
    filter:
      include:
        channel: ["Organic Search", "Direct", "Paid Search"]
    width: 1600
    height: 900
    theme: light
    
  # Stacked bar chart with derived columns
  - type: bar-stacked
    title: "Weekly Events by Type"
    x: week_start
    y: eventCount
    group_by: eventName
    agg: sum
    derive:
      week_start: "to_week(date)"
    filter:
      exclude:
        eventName: ["session_start", "first_visit"]
    
  # Heatmap from different data source
  - type: heatmap
    title: "Session Intensity by Hour and Weekday"
    data: hourly_sessions.csv
    x: hour
    y: weekday  
    z: sessions
    colormap: plasma
    bins: 15
    width: 1200
    height: 600
    
  # Funnel analysis
  - type: funnel
    title: "E-commerce Conversion Funnel"
    data: events
    steps: ["page_view", "add_to_cart", "begin_checkout", "purchase"]
    values: eventCount
    conversion_rates: true
```

## Filter Expressions

### Include/Exclude Filters

```yaml
filter:
  include:
    column_name: ["value1", "value2"]  # Column must be in list
    another_column: "single_value"     # Column must equal value
  exclude:
    column_name: ["unwanted1", "unwanted2"]  # Column must not be in list
```

### Expression Filters

Support SQL-like expressions for complex filtering:

```yaml
filter:
  expression: "totalUsers > 100 AND sessions < 1000"
```

#### Supported Operators
- Comparison: `>`, `>=`, `<`, `<=`, `=`, `!=`
- Logical: `AND`, `OR`, `NOT`
- String: `LIKE`, `IN`, `NOT IN`
- Null checks: `IS NULL`, `IS NOT NULL`

#### Examples
```yaml
# Numeric comparisons
expression: "totalUsers > 100"
expression: "sessions BETWEEN 10 AND 1000"

# String matching
expression: "channel LIKE '%Search%'"
expression: "eventName IN ('purchase', 'add_to_cart')"

# Date filtering
expression: "date >= '2023-01-01' AND date < '2024-01-01'"

# Complex conditions
expression: "(totalUsers > 100 OR sessions > 500) AND channel != 'spam'"
```

## Derived Column Functions

### Date/Time Functions
- `to_week(date_column)` - Convert to Monday week start
- `to_month(date_column)` - Convert to first day of month
- `to_quarter(date_column)` - Convert to first day of quarter
- `to_year(date_column)` - Extract year
- `to_hour(timestamp_column)` - Extract hour (0-23)
- `weekday(date_column)` - Day of week (0=Monday, 6=Sunday)
- `weekday_name(date_column)` - Day name ("Monday", "Tuesday", ...)

### String Functions  
- `source_medium(source, medium)` - Combine as "source / medium"
- `concat(col1, col2, separator)` - Concatenate columns with separator
- `upper(column)` - Convert to uppercase
- `lower(column)` - Convert to lowercase

### Numeric Functions
- `round(column, decimals)` - Round to specified decimal places
- `abs(column)` - Absolute value
- `log(column)` - Natural logarithm
- `sqrt(column)` - Square root

### Examples
```yaml
derive:
  # Date transformations
  week_start: "to_week(date)"
  month_year: "to_month(date)"
  hour_of_day: "to_hour(event_timestamp)"
  day_name: "weekday_name(date)"
  
  # String operations
  traffic_source: "source_medium(utm_source, utm_medium)"
  full_name: "concat(first_name, last_name, ' ')"
  
  # Numeric calculations
  conversion_rate: "round(conversions / sessions * 100, 2)"
  log_users: "log(totalUsers + 1)"
```

## Error Handling

### Common Error Types

#### Column Not Found
```
Error: Column 'totalUser' not found in CSV
Available columns: date, totalUsers, sessions, channel
Did you mean 'totalUsers'?
```

#### Invalid Aggregation
```
Error: Invalid aggregation 'average' for chart type 'line'
Supported aggregations: sum, count, mean, median, min, max
```

#### Date Parsing Error
```
Error: Could not parse date in column 'event_date'
Expected formats: YYYYMMDD (20231225) or ISO (2023-12-25)
Found: '2023/12/25' at row 42
```

#### Spec Validation Error
```
Error: Chart specification invalid
  - charts[0].x: field is required
  - charts[1].type: 'invalid_type' is not a valid chart type
  - charts[2].steps: field is required for funnel charts
```

### Exit Codes

- `0` - Success
- `1` - General error (invalid arguments, file not found, etc.)
- `2` - Data processing error (CSV parsing, column issues, etc.)
- `3` - Chart rendering error (invalid data, plotting failure, etc.)
- `4` - Specification error (invalid YAML/JSON, missing required fields, etc.)

## Output File Naming

When `--out` is not specified, Graff generates deterministic file names:

### Single Chart Commands
Pattern: `{chart_type}-{timestamp}.{format}`

Examples:
- `line-20231225143022.png`
- `heatmap-20231225143045.svg`

### Batch Rendering
Pattern: `{title_slug}-{chart_type}.{format}`

Examples:
- `daily-users-by-channel-line.png`
- `session-heatmap-heatmap.png`
- `conversion-funnel-funnel.svg`

Title slugging rules:
- Convert to lowercase
- Replace spaces with hyphens  
- Remove special characters
- Truncate to 50 characters
