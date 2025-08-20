use crate::spec::{ChartConfig, ChartType, OutputFormat};
use anyhow::{Context, Result};
use plotters::prelude::*;
use polars::prelude::*;
use std::path::Path;

pub mod styling;

pub fn render_chart(data: LazyFrame, config: &ChartConfig, output_path: &Path) -> Result<()> {
    // Collect the data for rendering
    let df = data
        .collect()
        .context("Failed to collect data for rendering")?;

    // Calculate dimensions
    let width = config.width.unwrap_or(800);
    let height = config.height.unwrap_or(600);
    let scaled_width = (width as f32 * 1.0) as u32;
    let scaled_height = (height as f32 * 1.0) as u32;

    // Render based on output format
    match config.format.as_ref().unwrap_or(&OutputFormat::Png) {
        OutputFormat::Png => {
            render_to_bitmap(&df, config, output_path, scaled_width, scaled_height)
        }
        OutputFormat::Svg => render_to_svg(&df, config, output_path, scaled_width, scaled_height),
        OutputFormat::Pdf => {
            // For now, render as PNG for PDF (could be enhanced later)
            render_to_bitmap(&df, config, output_path, scaled_width, scaled_height)
        }
    }
}

fn render_to_bitmap(
    df: &DataFrame,
    config: &ChartConfig,
    output_path: &Path,
    width: u32,
    height: u32,
) -> Result<()> {
    let backend = BitMapBackend::new(output_path, (width, height)).into_drawing_area();
    render_chart_impl(df, config, backend)
}

fn render_to_svg(
    df: &DataFrame,
    config: &ChartConfig,
    output_path: &Path,
    width: u32,
    height: u32,
) -> Result<()> {
    let backend = SVGBackend::new(output_path, (width, height)).into_drawing_area();
    render_chart_impl(df, config, backend)
}

fn render_chart_impl<DB: DrawingBackend>(
    df: &DataFrame,
    config: &ChartConfig,
    root: DrawingArea<DB, plotters::coord::Shift>,
) -> Result<()>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    // Get theme from config or default to light
    let theme = config.theme.as_ref().unwrap_or(&crate::spec::Theme::Light);
    let style = crate::render::styling::get_chart_style_with_theme(theme);

    // Fill with theme-appropriate background
    root.fill(&style.colors.background.canvas)
        .context("Failed to fill background")?;

    // Get the title
    let title = config.title.as_deref().unwrap_or("Chart");

    // Get legend position (default to Right if not specified)
    let legend_position = config
        .legend_position
        .as_ref()
        .unwrap_or(&crate::spec::LegendPosition::Right);

    // Split the drawing area based on legend position
    let (chart_area, legend_area) = split_drawing_area(&root, legend_position)?;

    // Render the chart in the chart area
    match config.chart_type {
        ChartType::Line => {
            crate::chart::line::render(df, config, chart_area, title, legend_position)
        }
        ChartType::Area => {
            crate::chart::area::render(df, config, chart_area, title, legend_position)
        }
        ChartType::Bar => crate::chart::bar::render(df, config, chart_area, title, legend_position),
        ChartType::BarStacked => {
            crate::chart::bar_stacked::render(df, config, chart_area, title, legend_position)
        }
        ChartType::Heatmap => {
            crate::chart::heatmap::render(df, config, chart_area, title, legend_position)
        }
        ChartType::Scatter => {
            crate::chart::scatter::render(df, config, chart_area, title, legend_position)
        }
        ChartType::Funnel => {
            crate::chart::funnel::render(df, config, chart_area, title, legend_position)
        }
        ChartType::Retention => {
            crate::chart::retention::render(df, config, chart_area, title, legend_position)
        }
    }?;

    // Render the legend in the legend area
    render_external_legend(df, config, legend_area, legend_position)?;

    Ok(())
}

fn split_drawing_area<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    legend_position: &crate::spec::LegendPosition,
) -> Result<(
    DrawingArea<DB, plotters::coord::Shift>,
    DrawingArea<DB, plotters::coord::Shift>,
)>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    let (width, height) = root.dim_in_pixel();

    // Adjust legend space based on position - horizontal legends need more width
    let legend_width = match legend_position {
        crate::spec::LegendPosition::Right | crate::spec::LegendPosition::Left => {
            (width as f32 * 0.25) as u32 // 25% for horizontal legends
        }
        _ => (width as f32 * 0.15) as u32, // 15% for vertical legends
    };
    let legend_height = (height as f32 * 0.15) as u32; // 15% of height for legend

    match legend_position {
        crate::spec::LegendPosition::Right => {
            let (left, right) = root.split_horizontally(width - legend_width);
            Ok((left, right))
        }
        crate::spec::LegendPosition::Left => {
            let (left, right) = root.split_horizontally(legend_width);
            Ok((right, left))
        }
        crate::spec::LegendPosition::Top => {
            let (top, bottom) = root.split_vertically(legend_height);
            Ok((bottom, top))
        }
        crate::spec::LegendPosition::Bottom => {
            let (top, bottom) = root.split_vertically(height - legend_height);
            Ok((top, bottom))
        }
    }
}

fn render_external_legend<DB: DrawingBackend>(
    df: &DataFrame,
    config: &ChartConfig,
    legend_area: DrawingArea<DB, plotters::coord::Shift>,
    _legend_position: &crate::spec::LegendPosition,
) -> Result<()>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    // Get theme from config or default to light
    let theme = config.theme.as_ref().unwrap_or(&crate::spec::Theme::Light);
    let style = crate::render::styling::get_chart_style_with_theme(theme);

    // Fill legend area with theme-appropriate background
    legend_area
        .fill(&style.colors.background.chart)
        .context("Failed to fill legend background")?;

    // Get legend items based on chart type
    let legend_items = get_legend_items(df, config)?;

    // Get legend area dimensions for better text handling
    let (legend_width, _legend_height) = legend_area.dim_in_pixel();

    // Render legend items
    let style = crate::render::styling::get_chart_style();
    let mut y_offset = 30; // Start 30 pixels from top for better spacing

    for (index, item) in legend_items.iter().enumerate() {
        let color = style.get_primary_color(index);

        // Draw legend symbol
        legend_area
            .draw(&Rectangle::new(
                [(15, y_offset), (35, y_offset + 15)],
                color.filled(),
            ))
            .context("Failed to draw legend symbol")?;

        // Calculate available text width (legend width minus symbol and padding)
        let available_width = legend_width.saturating_sub(60); // 60px for symbol + padding
        let max_chars = (available_width as f32 / 8.0) as usize; // Approximate chars per pixel

        // Truncate text based on available space
        let display_text = if item.len() > max_chars && max_chars > 10 {
            format!("{}...", &item[..max_chars.saturating_sub(3)])
        } else {
            item.clone()
        };

        // Draw legend text with better positioning
        legend_area
            .draw(&Text::new(
                display_text.as_str(),
                (45, y_offset + 12),
                style.axis_label_font(),
            ))
            .context("Failed to draw legend text")?;

        y_offset += 35; // Increase spacing between legend items
    }

    Ok(())
}

fn get_legend_items(df: &DataFrame, config: &ChartConfig) -> Result<Vec<String>> {
    let mut items = Vec::new();

    match config.chart_type {
        ChartType::Line => {
            if let Some(y) = &config.y {
                items.push(y.clone());
            }
        }
        ChartType::Area => {
            if let Some(y) = &config.y {
                items.push(y.clone());
            }
        }
        ChartType::Bar => {
            if let Some(y) = &config.y {
                items.push(y.clone());
            }
        }
        ChartType::BarStacked => {
            if let Some(group_by) = &config.group_by {
                // Get unique values from group_by column
                if let Ok(group_col) = df.column(group_by) {
                    let mut unique_groups = std::collections::HashSet::new();
                    for i in 0..df.height().min(50) {
                        if let Ok(val) = group_col.get(i) {
                            unique_groups.insert(format!("{:?}", val));
                        }
                    }
                    items.extend(unique_groups.into_iter().collect::<Vec<_>>());
                }
            }
        }
        ChartType::Scatter => {
            if let (Some(x), Some(y)) = (&config.x, &config.y) {
                items.push(format!("{} vs {}", y, x));
            }
        }
        ChartType::Funnel => {
            if let Some(steps) = &config.steps {
                // Apply step ordering if specified
                if let Some(step_order) = &config.step_order {
                    // Validate and apply order
                    if step_order.len() == steps.len() {
                        let ordered_steps: Vec<String> = step_order
                            .iter()
                            .filter_map(|&idx| steps.get(idx).cloned())
                            .collect();
                        items.extend(ordered_steps);
                    } else {
                        items.extend(steps.clone());
                    }
                } else {
                    // Default order: use steps as provided
                    items.extend(steps.clone());
                }
            }
        }
        ChartType::Retention => {
            items.push("Retention %".to_string());
        }
        _ => {
            if let Some(y) = &config.y {
                items.push(y.clone());
            }
        }
    }

    Ok(items)
}

pub fn generate_output_filename(
    config: &ChartConfig,
    output_dir: &Path,
) -> Result<std::path::PathBuf> {
    let title = config.title.as_deref().unwrap_or("chart");
    let chart_type = match config.chart_type {
        ChartType::Line => "Line",
        ChartType::Area => "Area",
        ChartType::Bar => "Bar",
        ChartType::BarStacked => "BarStacked",
        ChartType::Heatmap => "Heatmap",
        ChartType::Scatter => "Scatter",
        ChartType::Funnel => "Funnel",
        ChartType::Retention => "Retention",
    };
    let format = match config.format.as_ref().unwrap_or(&OutputFormat::Png) {
        OutputFormat::Png => "png",
        OutputFormat::Svg => "svg",
        OutputFormat::Pdf => "pdf",
    };

    // Sanitize the title for filename
    let safe_title = title
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .to_lowercase();

    let filename = format!("{}-{}.{}", safe_title, chart_type, format);
    Ok(output_dir.join(filename))
}
