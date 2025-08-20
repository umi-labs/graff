use crate::render::styling::get_chart_style;
use crate::spec::{ChartConfig, LegendPosition};
use anyhow::{Context, Result};
use plotters::prelude::*;
use polars::prelude::*;

pub fn render<DB: DrawingBackend>(
    df: &DataFrame,
    config: &ChartConfig,
    root: DrawingArea<DB, plotters::coord::Shift>,
    title: &str,
    _legend_position: &LegendPosition,
) -> Result<()>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    let x_col = df
        .column(config.x.as_ref().unwrap())
        .context("X column not found")?;
    let y_col = df
        .column(config.y.as_ref().unwrap())
        .context("Y column not found")?;

    let mut data_points = Vec::new();

    for i in 0..df.height().min(1000) {
        // Limit points for performance but allow more than other charts
        if let (Ok(x_val), Ok(y_val)) = (x_col.get(i), y_col.get(i)) {
            let x = extract_numeric_value(x_val).unwrap_or(i as f32);
            let y = extract_numeric_value(y_val).unwrap_or(0.0);
            data_points.push((x, y));
        }
    }

    if data_points.is_empty() {
        return Ok(());
    }

    // Calculate ranges with padding
    let x_min = data_points
        .iter()
        .map(|(x, _)| *x)
        .fold(f32::INFINITY, f32::min);
    let x_max = data_points
        .iter()
        .map(|(x, _)| *x)
        .fold(f32::NEG_INFINITY, f32::max);
    let y_min = data_points
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::INFINITY, f32::min);
    let y_max = data_points
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::NEG_INFINITY, f32::max);

    // Add 10% padding to ranges
    let x_range = {
        let padding = (x_max - x_min) * 0.1;
        (x_min - padding)..(x_max + padding)
    };
    let y_range = {
        let padding = (y_max - y_min) * 0.1;
        (y_min - padding)..(y_max + padding)
    };

    let style = get_chart_style();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(x_range, y_range)
        .context("Failed to build chart")?;

    chart
        .configure_mesh()
        .y_desc(config.y.as_ref().unwrap())
        .x_desc(config.x.as_ref().unwrap())
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    // Use the primary color for scatter points
    let point_color = style.get_primary_color(0);
    let point_size = style.layout.elements.line_points; // Reuse line point size

    // Draw scatter points
    chart
        .draw_series(
            data_points
                .iter()
                .map(|(x, y)| Circle::new((*x, *y), point_size, point_color.filled())),
        )
        .context("Failed to draw scatter points")?
        .label(format!(
            "{} vs {}",
            config.y.as_ref().unwrap(),
            config.x.as_ref().unwrap()
        ))
        .legend(|(x, y)| {
            Circle::new(
                (x + 5, y),
                style.layout.elements.line_points,
                point_color.filled(),
            )
        });

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
