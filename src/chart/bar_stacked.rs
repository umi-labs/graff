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
    
    // For stacked bars, we need both x and group_by columns
    let group_by_col = config.group_by.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Stacked bar charts require a 'group_by' field"))?;
    
    // Check if we have the original x column or if we need to use the group column
    if let Ok(_) = df.column(config.x.as_ref().unwrap()) {
        render_stacked_bar_with_x(df, config, root, title, group_by_col, &style)
    } else {
        render_stacked_bar_grouped(df, config, root, title, group_by_col, &style)
    }
}

fn render_stacked_bar_with_x<DB: DrawingBackend>(
    df: &DataFrame,
    config: &ChartConfig,
    root: DrawingArea<DB, plotters::coord::Shift>,
    title: &str,
    group_by_col: &str,
    style: &crate::render::styling::ChartStyle,
) -> Result<()>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    // For stacked bars, we need both x and group_by columns
    let x_col = df.column(config.x.as_ref().unwrap()).context("X column not found")?;
    let y_col = df.column(config.y.as_ref().unwrap()).context("Y column not found")?;
    let group_col = df.column(group_by_col).context("Group column not found")?;

    // Collect data and organize by x categories and groups
    let mut category_data: std::collections::HashMap<String, std::collections::HashMap<String, f32>> = std::collections::HashMap::new();
    let mut all_groups = std::collections::HashSet::new();
    let mut categories = Vec::new();

    for i in 0..df.height().min(50) { // Limit for performance
        if let (Ok(x_val), Ok(y_val), Ok(group_val)) = (x_col.get(i), y_col.get(i), group_col.get(i)) {
            let x_str = format!("{:?}", x_val);
            let group_str = format!("{:?}", group_val);
            let y = extract_numeric_value(y_val).unwrap_or(0.0);
            
            category_data.entry(x_str.clone()).or_default().insert(group_str.clone(), y);
            all_groups.insert(group_str);
            
            if !categories.contains(&x_str) {
                categories.push(x_str);
            }
        }
    }

    if categories.is_empty() {
        return Ok(());
    }

    // Convert groups to sorted vector for consistent ordering
    let mut groups: Vec<String> = all_groups.into_iter().collect();
    groups.sort();

    // Calculate stacked values for each category
    let mut stacked_data = Vec::new();
    for (cat_idx, category) in categories.iter().enumerate() {
        let mut current_stack = 0.0;
        let mut category_stacks = Vec::new();
        
        for group in &groups {
            let value = category_data.get(category)
                .and_then(|cat_map| cat_map.get(group))
                .unwrap_or(&0.0);
            
            category_stacks.push((current_stack, current_stack + value));
            current_stack += value;
        }
        
        stacked_data.push((cat_idx, category_stacks));
    }

    // Find the maximum total height for scaling
    let max_height = stacked_data.iter()
        .map(|(_, stacks)| stacks.last().map(|(_, end)| *end).unwrap_or(0.0))
        .fold(0.0f32, f32::max);

    if max_height == 0.0 {
        return Ok(());
    }

    let y_range = 0f32..(max_height * 1.1);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(0usize..categories.len(), y_range)
        .context("Failed to build chart")?;

    chart.configure_mesh()
        .y_desc(config.y.as_ref().unwrap())
        .x_desc(config.x.as_ref().unwrap())
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    // Draw stacked bars for each group
    for (group_idx, group) in groups.iter().enumerate() {
        let color = style.get_primary_color(group_idx);
        
        chart
            .draw_series(
                stacked_data
                    .iter()
                    .map(|(cat_idx, stacks)| {
                        let (start, end) = stacks[group_idx];
                        Rectangle::new([(*cat_idx, start), (cat_idx + 1, end)], color.filled())
                    })
            )
            .context("Failed to draw stacked bar series")?
            .label(group)
            .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], color.filled()));
    }

    // Legend is now handled externally

    root.present().context("Failed to present chart")?;
    Ok(())
}

fn render_stacked_bar_grouped<DB: DrawingBackend>(
    df: &DataFrame,
    config: &ChartConfig,
    root: DrawingArea<DB, plotters::coord::Shift>,
    title: &str,
    group_by_col: &str,
    style: &crate::render::styling::ChartStyle,
) -> Result<()>
where
    DB::ErrorType: 'static + std::error::Error + Send + Sync,
{
    // For grouped data, we need to handle the structure differently
    let group_col = df.column(group_by_col).context("Group column not found")?;
    let value_col = df.column(config.y.as_ref().unwrap()).context("Value column not found")?;

    // Collect data and organize by groups
    let mut group_data: std::collections::HashMap<String, f32> = std::collections::HashMap::new();
    let mut groups = Vec::new();

    for i in 0..df.height().min(50) { // Limit for performance
        if let (Ok(group_val), Ok(value_val)) = (group_col.get(i), value_col.get(i)) {
            let group_str = format!("{:?}", group_val);
            let value = extract_numeric_value(value_val).unwrap_or(0.0);
            
            group_data.insert(group_str.clone(), value);
            if !groups.contains(&group_str) {
                groups.push(group_str);
            }
        }
    }

    if groups.is_empty() {
        return Ok(());
    }

    // Sort groups for consistent ordering
    groups.sort();

    // Calculate stacked values
    let mut stacked_data = Vec::new();
    let mut current_stack = 0.0;
    
    for group in &groups {
        let value = group_data.get(group).unwrap_or(&0.0);
        stacked_data.push((current_stack, current_stack + value));
        current_stack += value;
    }

    let max_height = stacked_data.last().map(|(_, end)| *end).unwrap_or(0.0);
    if max_height == 0.0 {
        return Ok(());
    }

    let y_range = 0f32..(max_height * 1.1);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(0usize..1, y_range)
        .context("Failed to build chart")?;

    chart.configure_mesh()
        .y_desc(config.y.as_ref().unwrap())
        .x_desc(group_by_col)
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    // Draw stacked bars for each group
    for (group_idx, group) in groups.iter().enumerate() {
        let color = style.get_primary_color(group_idx);
        let (start, end) = stacked_data[group_idx];
        
        chart
            .draw_series(std::iter::once(
                Rectangle::new([(0, start), (1, end)], color.filled())
            ))
            .context("Failed to draw stacked bar series")?
            .label(group)
            .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], color.filled()));
    }

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
