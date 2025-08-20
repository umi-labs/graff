# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive chart generation system with 8 chart types
- Light and dark theme support for all charts
- GitHub workflow for automatic example generation
- Release automation script with cargo dist integration
- Comprehensive documentation with examples

### Changed
- Moved theme styling from palette.rs to styling.rs
- Improved chart rendering with grouped data support
- Enhanced error handling and validation

### Fixed
- Fixed grouped data handling in line, area, bar, and stacked bar charts
- Resolved compilation warnings and unused imports
- Fixed funnel chart interactive input for CI environments

## [0.1.0] - 2024-08-20

### Added
- Initial release of Graff chart generation tool
- Support for 8 chart types: Line, Area, Bar, Stacked Bar, Heatmap, Scatter, Funnel, Retention
- Command-line interface with comprehensive options
- CSV data loading and processing
- Chart rendering with Plotters library
- Theme support (light/dark)
- Colorblind-friendly color palettes
- Comprehensive error handling
- Documentation and examples

### Features
- **Line Charts**: Time series and trend analysis with grouping support
- **Area Charts**: Composition and stacked data visualization
- **Bar Charts**: Categorical comparisons with horizontal and grouped options
- **Stacked Bar Charts**: Composition analysis
- **Heatmaps**: 2D data visualization with custom colormaps
- **Scatter Plots**: Correlation analysis with categorization
- **Funnel Charts**: Conversion analysis with configurable step ordering
- **Retention Charts**: Cohort analysis with percentage support

### Technical
- Built with Rust for performance and reliability
- Uses Polars for efficient data processing
- Plotters library for high-quality chart rendering
- Comprehensive test suite
- CI/CD integration with GitHub Actions
