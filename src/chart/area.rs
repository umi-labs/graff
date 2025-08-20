use anyhow::{Context, Result};
use polars::prelude::*;
use plotters::prelude::*;
use crate::spec::{ChartConfig, LegendPosition};
use crate::render::styling::get_chart_style;

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
    let style = get_chart_style();
    
    // Check if we have grouped data
    if let Some(group_by) = &config.group_by {
        render_grouped_area_chart(df, config, root, title, group_by, &style)
    } else {
        render_simple_area_chart(df, config, root, title, &style)
    }
}

fn render_simple_area_chart<DB: DrawingBackend>(
    df: &DataFrame,
    config: &ChartConfig,
    root: DrawingArea<DB, plotters::coord::Shift>,
    title: &str,
    style: &crate::render::styling::ChartStyle,
) -> Result<()>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    let x_col = df.column(config.x.as_ref().unwrap()).context("X column not found")?;
    let y_col = df.column(config.y.as_ref().unwrap()).context("Y column not found")?;

    let mut data_points = Vec::new();

    for i in 0..df.height().min(100) { // Limit points for performance
        if let (Ok(_x_val), Ok(y_val)) = (x_col.get(i), y_col.get(i)) {
            let y = extract_numeric_value(y_val).unwrap_or(0.0);
            data_points.push((i as f32, y));
        }
    }

    if data_points.is_empty() {
        return Ok(());
    }

    let x_range = 0f32..data_points.len() as f32;
    let y_max = data_points.iter().map(|(_, y)| *y).fold(0.0f32, f32::max);
    let y_range = 0f32..(y_max * 1.1); // Add 10% padding

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(x_range, y_range)
        .context("Failed to build chart")?;

    chart.configure_mesh()
        .x_desc(config.x.as_ref().unwrap())
        .y_desc(config.y.as_ref().unwrap())
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    // Get the primary color and create a semi-transparent fill
    let line_color = style.get_primary_color(0);
    
    // Create area data points (filled down to zero)
    let area_points: Vec<(f32, f32)> = data_points.iter().cloned().collect();
    let area_fill = RGBColor(line_color.0, line_color.1, line_color.2).mix(0.3);

    // Draw the filled area using polygon
    chart
        .draw_series(
            area_points
                .windows(2)
                .map(|window| {
                    let (x1, y1) = window[0];
                    let (x2, y2) = window[1];
                    Polygon::new(vec![(x1, 0.0), (x1, y1), (x2, y2), (x2, 0.0)], area_fill)
                })
        )
        .context("Failed to draw area series")?
        .label(config.y.as_ref().unwrap())
        .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], RGBColor(line_color.0, line_color.1, line_color.2).mix(0.3)));

    // Draw the line on top of the area for better definition
    chart
        .draw_series(LineSeries::new(data_points.iter().cloned(), line_color))
        .context("Failed to draw line series")?;

    // Legend is now handled externally
    
    root.present().context("Failed to present chart")?;
    Ok(())
}

fn render_grouped_area_chart<DB: DrawingBackend>(
    df: &DataFrame,
    config: &ChartConfig,
    root: DrawingArea<DB, plotters::coord::Shift>,
    title: &str,
    group_by: &str,
    style: &crate::render::styling::ChartStyle,
) -> Result<()>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    // For grouped data, we need to handle the structure differently
    let group_col = df.column(group_by).context("Group column not found")?;
    let value_col = df.column(config.y.as_ref().unwrap()).context("Value column not found")?;

    let mut data_points = Vec::new();

    for i in 0..df.height().min(100) { // Limit points for performance
        if let (Ok(_group_val), Ok(value_val)) = (group_col.get(i), value_col.get(i)) {
            let y = extract_numeric_value(value_val).unwrap_or(0.0);
            data_points.push((i as f32, y));
        }
    }

    if data_points.is_empty() {
        return Ok(());
    }

    let x_range = 0f32..data_points.len() as f32;
    let y_max = data_points.iter().map(|(_, y)| *y).fold(0.0f32, f32::max);
    let y_range = 0f32..(y_max * 1.1); // Add 10% padding

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(x_range, y_range)
        .context("Failed to build chart")?;

    chart.configure_mesh()
        .x_desc(group_by)
        .y_desc(config.y.as_ref().unwrap())
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    // Get the primary color and create a semi-transparent fill
    let line_color = style.get_primary_color(0);
    
    // Create area data points (filled down to zero)
    let area_points: Vec<(f32, f32)> = data_points.iter().cloned().collect();
    let area_fill = RGBColor(line_color.0, line_color.1, line_color.2).mix(0.3);

    // Draw the filled area using polygon
    chart
        .draw_series(
            area_points
                .windows(2)
                .map(|window| {
                    let (x1, y1) = window[0];
                    let (x2, y2) = window[1];
                    Polygon::new(vec![(x1, 0.0), (x1, y1), (x2, y2), (x2, 0.0)], area_fill)
                })
        )
        .context("Failed to draw area series")?
        .label(config.y.as_ref().unwrap())
        .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], RGBColor(line_color.0, line_color.1, line_color.2).mix(0.3)));

    // Draw the line on top of the area for better definition
    chart
        .draw_series(LineSeries::new(data_points.iter().cloned(), line_color))
        .context("Failed to draw line series")?;

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