# Graff

A fast, deterministic Rust CLI tool for converting GA4/BigQuery CSV exports into beautiful PNG/SVG/PDF charts. Built for analytics teams who need reliable, reproducible visualizations in CI/CD pipelines.

## Features

- **Fast & Memory Efficient**: Uses Polars LazyFrame with streaming for large datasets
- **Multiple Chart Types**: Line, area, bar (grouped/stacked), heatmap, funnel, retention matrix
- **GA4/BigQuery Ready**: Auto-detects common date formats (YYYYMMDD, ISO) and column patterns
- **Batch Processing**: YAML/JSON specs for rendering multiple charts at once
- **Deterministic Output**: Same input always produces identical images
- **CI/CD Friendly**: Headless operation, clear error messages, non-zero exit codes

## Quick Start

### Installation

```bash
cargo install graff
```

### Basic Usage

```bash
# Simple line chart
graff line --input ga4_users.csv --x date --y totalUsers --out users.png

# Grouped bar chart
graff bar --input ga4_events.csv --x date --y eventCount --group eventName --agg sum --out events.png

# Batch render from spec
graff render --spec charts.yaml --data ga4_users.csv --out ./charts
```

## Chart Types

### Line Charts
Perfect for time series data showing trends over time.

```bash
graff line \
  --input ga4_users.csv \
  --x date \
  --y totalUsers \
  --group channel \
  --agg sum \
  --title "Users by Channel" \
  --out users_by_channel.png
```

### Bar Charts
Great for comparing categories, supports both grouped and stacked modes.

```bash
# Grouped bars
graff bar --input data.csv --x category --y value --group segment --out grouped.png

# Stacked bars
graff bar --input data.csv --x category --y value --group segment --stacked --out stacked.png
```

### Heatmaps
Ideal for showing patterns across two dimensions (e.g., hour vs day of week).

```bash
graff heatmap \
  --input hourly.csv \
  --x hour \
  --y weekday \
  --z sessions \
  --out hourly_heatmap.png
```

### Funnels
Track conversion rates through sequential steps.

```bash
graff funnel \
  --input funnel.csv \
  --steps page_view add_to_cart purchase \
  --values eventCount \
  --out funnel.png
```

### Retention Matrix
Analyze user retention cohorts over time.

```bash
graff retention \
  --input retention.csv \
  --cohort_date first_seen \
  --period_number week_number \
  --users active_users \
  --out retention_matrix.png
```

## Spec Files

For complex workflows, use YAML/JSON spec files to define multiple charts:

## Development & Release

### Prerequisites

```bash
# Install development dependencies
cargo install cargo-watch  # For development
cargo install cargo-audit  # For security checks
```

### Development Workflow

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run lints
cargo clippy --release -- -D warnings

# Build release
cargo build --release
```

### Release Process

Graff uses automated releases with cargo dist. To create a new release:

```bash
# Automatic version bumping (recommended)
./scripts/release.sh --patch          # 0.1.0 -> 0.1.1
./scripts/release.sh --minor          # 0.1.0 -> 0.2.0
./scripts/release.sh --major          # 0.1.0 -> 1.0.0

# Manual version specification
./scripts/release.sh 1.0.0

# Dry run to see what would happen
./scripts/release.sh --dry-run --minor

# Skip tests (for hotfixes)
./scripts/release.sh --patch --skip-tests
```

The release script will:
1. ✅ Auto-bump version (if using --patch/--minor/--major)
2. ✅ Run all tests and lints
3. ✅ Build the release version
4. ✅ Update version in Cargo.toml
5. ✅ Create git tag and push to GitHub

For detailed release instructions, see [docs/release-guide.md](docs/release-guide.md).

### Examples Generation

Generate comprehensive chart examples and documentation:

```bash
# Generate all examples
./scripts/generate-examples.sh

# Generate documentation
./scripts/generate-docs.sh
```

This creates:
- Chart examples in `charts/` directory
- Comprehensive documentation in `docs/examples.md`
- Both light and dark theme variations

```yaml
# charts.yaml
data:
  default: ga4.csv

charts:
  - type: line
    title: "Daily Users by Channel"
    x: date
    y: totalUsers
    group_by: channel
    agg: sum
    filter:
      include:
        channel: ["Organic Search", "Direct", "Paid Search"]
    width: 1400
    height: 800
    theme: light
    
  - type: bar-stacked
    title: "Weekly Event Count"
    x: week_start
    y: eventCount
    group_by: eventName
    derive:
      week_start: "to_week(date)"
    agg: sum
    filter:
      exclude:
        eventName: ["session_start"]
    
  - type: heatmap
    title: "Sessions by Hour and Weekday"
    data: hourly_sessions.csv
    x: hour
    y: weekday
    z: sessions
    width: 1200
    height: 600
```

Run with:
```bash
graff render --spec charts.yaml --out ./output
```

## GA4/BigQuery Support

Graff is designed specifically for GA4 and BigQuery exports with built-in support for:

### Date Formats
- **GA4 event_date**: `20231225` (YYYYMMDD) - auto-detected
- **BigQuery date**: `2023-12-25` (ISO format) - auto-detected
- **Timestamps**: UNIX microseconds with `to_date()` and `to_hour()` helpers

### Common Columns
- `totalUsers`, `sessions`, `eventCount`
- `deviceCategory`, `channel`, `eventName`
- `source`, `medium` (with `source_medium()` helper)

### Derived Columns
Built-in functions for common transformations:

```yaml
derive:
  week_start: "to_week(date)"          # Monday week start
  month_start: "to_month(date)"        # First of month
  hour_of_day: "to_hour(timestamp)"    # 0-23 hour
  day_of_week: "weekday(date)"         # Mon-Sun
  traffic_source: "source_medium(source, medium)"  # "google / organic"
```

### Large Dataset Handling
- Uses Polars LazyFrame for memory efficiency
- Streaming processing for files > 1GB
- Automatic schema inference with override options

## Configuration

### Themes
- `light` (default): Clean white background
- `dark`: Dark background for presentations
- Both use colorblind-friendly palettes

### Output Formats
- `png` (default): Best for web and reports
- `svg`: Vector format for scaling
- `pdf`: Print-ready documents

### Canvas Settings
- Default: 1400×800px at 1.0 scale
- Retina: Use `--scale 2.0` for high-DPI displays
- Custom: `--width 1920 --height 1080`

## CLI Reference

### Global Options
```
--verbose, -v     Enable verbose logging
--quiet, -q       Suppress all output except errors
--theme THEME     Chart theme: light, dark [default: light]
--scale SCALE     Canvas scale factor [default: 1.0]
--format FORMAT   Output format: png, svg, pdf [default: png]
```

### Commands

#### `line`
```
graff line [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN>

OPTIONS:
    --input <FILE>        Input CSV file
    --x <COLUMN>          X-axis column name
    --y <COLUMN>          Y-axis column name
    --group <COLUMN>      Group by column (creates multiple series)
    --agg <AGG>           Aggregation: sum, count, mean, min, max [default: sum]
    --filter <EXPR>       Filter expression (e.g., "channel='Organic'")
    --title <TITLE>       Chart title
    --out <FILE>          Output file path
    --width <WIDTH>       Canvas width [default: 1400]
    --height <HEIGHT>     Canvas height [default: 800]
```

#### `bar`
```
graff bar [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN>

OPTIONS:
    (same as line, plus:)
    --stacked             Create stacked bars instead of grouped
    --horizontal          Horizontal bar chart
```

#### `heatmap`
```
graff heatmap [OPTIONS] --input <FILE> --x <COLUMN> --y <COLUMN> --z <COLUMN>

OPTIONS:
    --input <FILE>        Input CSV file
    --x <COLUMN>          X-axis column name
    --y <COLUMN>          Y-axis column name  
    --z <COLUMN>          Value column name (for color intensity)
    --bins <N>            Number of color bins [default: 10]
    --colormap <MAP>      Color map: viridis, plasma, blues [default: viridis]
```

#### `funnel`
```
graff funnel [OPTIONS] --input <FILE> --steps <STEPS> --values <COLUMN>

OPTIONS:
    --input <FILE>        Input CSV file
    --steps <STEPS>       Comma-separated step names
    --values <COLUMN>     Value column name
    --conversion-rates    Show conversion rates between steps
```

#### `render`
```
graff render [OPTIONS] --spec <FILE>

OPTIONS:
    --spec <FILE>         YAML/JSON spec file
    --data <FILE>         Default data file (overrides spec default)
    --out <DIR>           Output directory [default: ./charts]
    --parallel <N>        Number of parallel renders [default: CPU cores]
```

## Error Handling

Graff provides clear, actionable error messages:

```bash
# Missing column
Error: Column 'totalUser' not found in CSV
Available columns: date, totalUsers, sessions, channel
Did you mean 'totalUsers'?

# Invalid aggregation
Error: Invalid aggregation 'average' for numeric data
Supported: sum, count, mean, median, min, max

# Date parsing issues  
Error: Could not parse date in column 'event_date'
Expected formats: YYYYMMDD (20231225) or ISO (2023-12-25)
Found: '2023/12/25'
```

## Performance Tips

1. **Use LazyFrame operations**: Filtering and grouping are optimized
2. **Batch processing**: Render multiple charts from one data load
3. **Streaming**: For files > 1GB, use `--streaming` flag
4. **Parallel rendering**: Spec files render charts concurrently
5. **Column selection**: Only load needed columns with `--select`

## Examples

See `docs/examples.md` for detailed real-world scenarios including:
- GA4 e-commerce funnel analysis
- Multi-channel attribution reporting
- Cohort retention analysis
- A/B test result visualization

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
