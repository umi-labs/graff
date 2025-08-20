# Graff Scripts

This directory contains utility scripts for the Graff chart generation tool.

## Scripts

### `generate-examples.sh`

Generates comprehensive examples for all chart types with various configurations and themes.

**Features:**
- Creates sample data for different chart types
- Generates charts in both light and dark themes
- Covers all 8 chart types with multiple variations
- Outputs charts to the `charts/` directory
- Cleans up temporary files automatically

**Usage:**
```bash
./scripts/generate-examples.sh
```

**Generated Examples:**
- **Line Charts**: Simple and multi-series with grouping
- **Area Charts**: Stacked and normalized compositions
- **Bar Charts**: Simple, grouped, and horizontal variations
- **Stacked Bar Charts**: Composition analysis
- **Heatmaps**: Session activity and custom colormaps
- **Scatter Plots**: Simple and categorized correlations
- **Funnel Charts**: Conversion funnels with different label positions
- **Retention Charts**: Matrix and percentage views

### `generate-docs.sh`

Generates comprehensive documentation showcasing all chart examples.

**Features:**
- Creates `docs/examples.md` with all chart examples
- Includes descriptions and command examples
- Shows both light and dark theme variations
- Provides usage instructions and best practices

**Usage:**
```bash
./scripts/generate-docs.sh
```

### `release.sh`

Comprehensive release automation script for cargo dist integration.

**Features:**
- Runs tests, lints, and builds
- Validates version format and git status
- Updates version in Cargo.toml
- Creates git tags and pushes to GitHub
- Integrates with cargo dist for distribution
- Supports dry-run mode for testing

**Usage:**
```bash
# Automatic version bumping (recommended)
./scripts/release.sh --patch          # 0.1.0 -> 0.1.1
./scripts/release.sh --minor          # 0.1.0 -> 0.2.0
./scripts/release.sh --major          # 0.1.0 -> 1.0.0

# Manual version specification
./scripts/release.sh 1.0.0

# Skip tests
./scripts/release.sh --patch --skip-tests

# Dry run to see what would happen
./scripts/release.sh --dry-run --minor

# Skip multiple steps
./scripts/release.sh --minor --skip-tests --skip-lints
```

**Version Options:**
- `--patch`: Bump patch version (1.0.0 -> 1.0.1)
- `--minor`: Bump minor version (1.0.0 -> 1.1.0)
- `--major`: Bump major version (1.0.0 -> 2.0.0)

**Other Options:**
- `--skip-tests`: Skip running tests
- `--skip-lints`: Skip running lints
- `--skip-build`: Skip building release
- `--skip-dist`: Skip cargo dist build (deprecated - not needed)
- `--dry-run`: Show what would be done without making changes
- `--help`: Show help message

## GitHub Workflow

The `.github/workflows/generate-examples.yml` workflow automatically:

1. **Builds** the Graff tool in release mode
2. **Clears** the charts directory
3. **Generates** all chart examples
4. **Creates** comprehensive documentation
5. **Commits** and pushes changes to the repository

**Triggers:**
- Push to main branch
- Pull requests to main branch
- Manual workflow dispatch

## Local Development

To test the scripts locally:

```bash
# Build the release version
cargo build --release

# Generate examples
./scripts/generate-examples.sh

# Generate documentation
./scripts/generate-docs.sh
```

## Chart Types Supported

1. **Line Charts** (`line`) - Time series and trends
2. **Area Charts** (`area`) - Composition and stacked data
3. **Bar Charts** (`bar`) - Categorical comparisons
4. **Stacked Bar Charts** (`bar-stacked`) - Composition analysis
5. **Heatmaps** (`heatmap`) - 2D data visualization
6. **Scatter Plots** (`scatter`) - Correlation analysis
7. **Funnel Charts** (`funnel`) - Conversion analysis
8. **Retention Charts** (`retention`) - Cohort analysis

## Theme Support

All charts support both light and dark themes:
- **Light Theme**: White backgrounds with dark text
- **Dark Theme**: Dark backgrounds with light text

## Output Structure

```
charts/
├── line-users-over-time-light.png
├── line-users-over-time-dark.png
├── area-user-composition-light.png
├── area-user-composition-dark.png
└── ... (32 total chart files)

docs/
└── examples.md  # Comprehensive documentation
```

## Requirements

- Rust toolchain
- Cargo package manager
- Bash shell (for script execution)
- Write permissions to `charts/` and `docs/` directories
