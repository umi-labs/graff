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
        render_grouped_bar_chart(df, config, root, title, group_by, &style)
    } else {
        render_simple_bar_chart(df, config, root, title, &style)
    }
}

fn render_simple_bar_chart<DB: DrawingBackend>(
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
    let mut _x_labels = Vec::new();
    
    for i in 0..df.height().min(20) { // Limit to 20 bars for readability
        if let (Ok(x_val), Ok(y_val)) = (x_col.get(i), y_col.get(i)) {
            let y = extract_numeric_value(y_val).unwrap_or(0.0);
            data_points.push((i, y));
            _x_labels.push(format!("{:?}", x_val));
        }
    }
    
    if data_points.is_empty() {
        return Ok(());
    }

    let y_max = data_points.iter().map(|(_, y)| *y).fold(0.0f32, f32::max);
    let y_range = 0f32..(y_max * 1.1);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(0usize..data_points.len(), y_range)
        .context("Failed to build chart")?;

    chart.configure_mesh()
        .y_desc(config.y.as_ref().unwrap())
        .x_desc(config.x.as_ref().unwrap())
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    chart
        .draw_series(
            data_points
                .iter()
                .enumerate()
                .map(|(i, (_, y))| {
                    let color = style.get_primary_color(i);
                    Rectangle::new([(i, 0.0), (i + 1, *y)], color.filled())
                }),
        )
        .context("Failed to draw bar series")?
        .label(config.y.as_ref().unwrap())
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], style.get_primary_color(0)));

    // Legend is now handled externally

    root.present().context("Failed to present chart")?;
    Ok(())
}

fn render_grouped_bar_chart<DB: DrawingBackend>(
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
    let mut _x_labels = Vec::new();
    
    for i in 0..df.height().min(20) { // Limit to 20 bars for readability
        if let (Ok(_group_val), Ok(value_val)) = (group_col.get(i), value_col.get(i)) {
            let y = extract_numeric_value(value_val).unwrap_or(0.0);
            data_points.push((i, y));
            _x_labels.push(format!("Group {}", i));
        }
    }
    
    if data_points.is_empty() {
        return Ok(());
    }

    let y_max = data_points.iter().map(|(_, y)| *y).fold(0.0f32, f32::max);
    let y_range = 0f32..(y_max * 1.1);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(0usize..data_points.len(), y_range)
        .context("Failed to build chart")?;

    chart.configure_mesh()
        .y_desc(config.y.as_ref().unwrap())
        .x_desc(group_by)
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    chart
        .draw_series(
            data_points
                .iter()
                .enumerate()
                .map(|(i, (_, y))| {
                    let color = style.get_primary_color(i);
                    Rectangle::new([(i, 0.0), (i + 1, *y)], color.filled())
                }),
        )
        .context("Failed to draw bar series")?
        .label(config.y.as_ref().unwrap())
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