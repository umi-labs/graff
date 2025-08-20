use crate::render::styling::{get_chart_style, get_heatmap_style};
use crate::spec::{ChartConfig, LegendPosition};
use anyhow::{Context, Result};
use plotters::prelude::*;
use polars::prelude::*;

pub fn render<DB: DrawingBackend>(
    df: &DataFrame,
    config: &ChartConfig,
    root: DrawingArea<DB, plotters::coord::Shift>,
    title: &str,
    legend_position: &LegendPosition,
) -> Result<()>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    // For a proper heatmap, we need x, y, and z columns
    // For now, if z is not specified, fallback to bar chart behavior
    if config.z.is_none() {
        return crate::chart::bar::render(df, config, root, title, legend_position);
    }

    let x_col = df
        .column(config.x.as_ref().unwrap())
        .context("X column not found")?;
    let y_col = df
        .column(config.y.as_ref().unwrap())
        .context("Y column not found")?;
    let z_col = df
        .column(config.z.as_ref().unwrap())
        .context("Z column not found")?;

    // For now, create a simplified heatmap using rectangles
    // In a full implementation, we'd create a proper grid
    let mut data_points = Vec::new();

    for i in 0..df.height().min(100) {
        // Limit for performance
        if let (Ok(_x_val), Ok(_y_val), Ok(z_val)) = (x_col.get(i), y_col.get(i), z_col.get(i))
            && let Some(z_value) = extract_numeric_value(z_val) {
            data_points.push((i, i, z_value)); // Simple mapping for now
        }
    }

    if data_points.is_empty() {
        return Ok(());
    }

    let z_max = data_points
        .iter()
        .map(|(_, _, z)| *z)
        .fold(0.0f32, f32::max);
    let z_min = data_points.iter().map(|(_, _, z)| *z).fold(z_max, f32::min);

    let style = get_chart_style();
    let heatmap_style = get_heatmap_style();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(0usize..data_points.len(), 0usize..data_points.len())
        .context("Failed to build chart")?;

    chart
        .configure_mesh()
        .y_desc(config.y.as_ref().unwrap())
        .x_desc(config.x.as_ref().unwrap())
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    // Draw heatmap rectangles with neutral color intensity based on z value
    chart
        .draw_series(data_points.iter().enumerate().map(|(i, (_, _, z))| {
            let intensity = if z_max > z_min {
                (z - z_min) / (z_max - z_min)
            } else {
                0.5
            };
            // Use the styled gradient colors
            let (_min_color, _max_color) = heatmap_style.gradient_colors;
            let base_color = heatmap_style.intensity_range.0
                + (intensity * (heatmap_style.intensity_range.1 - heatmap_style.intensity_range.0));
            let color = RGBColor(
                base_color as u8,
                (base_color * 1.1) as u8,
                (base_color * 1.2) as u8,
            );
            Rectangle::new([(i, i), (i + 1, i + 1)], color.filled())
        }))
        .context("Failed to draw heatmap series")?
        .label(config.z.as_ref().unwrap())
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], style.get_primary_color(0)));

    // Legend is now handled externally

    root.present().context("Failed to present chart")?;
    Ok(())
}

fn extract_numeric_value(value: AnyValue) -> Option<f32> {
    match value {
        AnyValue::Int32(i) => Some(i as f32),
        AnyValue::Int64(i) => Some(i as f32),
        AnyValue::Float32(f) => Some(f),
        AnyValue::Float64(f) => Some(f as f32),
        AnyValue::UInt32(u) => Some(u as f32),
        AnyValue::UInt64(u) => Some(u as f32),
        _ => None,
    }
}
