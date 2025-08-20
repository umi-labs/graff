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
    // For funnel charts, we need steps and values
    let steps = config
        .steps
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Funnel charts require a 'steps' field"))?;
    let values_col = config
        .values
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Funnel charts require a 'values' field"))?;

    let values_col_data = df.column(values_col).context("Values column not found")?;

    // Extract values for each step
    let mut step_values = Vec::new();
    for (step_idx, step) in steps.iter().enumerate() {
        if step_idx < df.height()
            && let Ok(value) = values_col_data.get(step_idx) {
            let numeric_value = extract_numeric_value(value).unwrap_or(0.0);
            step_values.push((step.clone(), numeric_value));
        }
    }

    if step_values.is_empty() {
        return Ok(());
    }

    // Apply step ordering if specified
    let ordered_step_values = if let Some(step_order) = &config.step_order {
        // Validate step order
        if step_order.len() != step_values.len() {
            anyhow::bail!(
                "Step order length ({}) must match number of steps ({})",
                step_order.len(),
                step_values.len()
            );
        }

        // Check for valid indices
        for &idx in step_order {
            if idx >= step_values.len() {
                anyhow::bail!(
                    "Invalid step order index: {} (max: {})",
                    idx,
                    step_values.len() - 1
                );
            }
        }

        // Reorder steps according to step_order
        step_order
            .iter()
            .map(|&idx| step_values[idx].clone())
            .collect()
    } else {
        // Default order: largest value first (top of funnel)
        let mut sorted = step_values.clone();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sorted
    };

    // Find the maximum value for scaling
    let max_value = ordered_step_values
        .iter()
        .map(|(_, value)| *value)
        .fold(0.0f32, f32::max);
    if max_value == 0.0 {
        return Ok(());
    }

    let style = get_chart_style();

    // Fill background with white (no grid/axes needed for funnel)
    root.fill(&WHITE).context("Failed to fill background")?;

    // Draw title
    root.draw(&Text::new(
        title,
        (root.dim_in_pixel().0 as i32 / 2 - 50, 20),
        style.title_font(),
    ))
    .context("Failed to draw title")?;

    // Calculate funnel dimensions (centered in the drawing area)
    let (width, height) = root.dim_in_pixel();
    let funnel_width = (width as f32 * 0.6) as u32; // 60% of width
    let funnel_height = (height as f32 * 0.6) as u32; // 60% of height
    let funnel_start_x = (width - funnel_width) / 2;
    let funnel_start_y = (height - funnel_height) / 2 + 50; // Add space for title

    // Draw funnel segments (widest at top, narrowest at bottom)
    let num_steps = ordered_step_values.len();
    let segment_height = funnel_height / num_steps as u32;

    for (step_idx, (step_name, value)) in ordered_step_values.iter().enumerate() {
        let color = style.get_primary_color(step_idx);

        // Calculate segment dimensions (top to bottom)
        let segment_y_start = funnel_start_y + (step_idx as u32 * segment_height);
        let segment_y_end = segment_y_start + segment_height;

        // Calculate width based on value (proportional to max_value)
        let width_ratio = value / max_value;
        let segment_width = (funnel_width as f32 * width_ratio) as u32;
        let segment_x_start = funnel_start_x + (funnel_width - segment_width) / 2;
        let segment_x_end = segment_x_start + segment_width;

        // Draw rectangle for funnel segment
        root.draw(&Rectangle::new(
            [
                (segment_x_start as i32, segment_y_start as i32),
                (segment_x_end as i32, segment_y_end as i32),
            ],
            color.filled(),
        ))
        .context("Failed to draw funnel segment")?;

        // Draw step label based on value_labels position
        let label_text = format!("{}: {:.0}", step_name, value);
        let (text_x, text_y) = match config
            .value_labels
            .as_ref()
            .unwrap_or(&crate::spec::ValueLabelPosition::Right)
        {
            crate::spec::ValueLabelPosition::Left => {
                // Position label on the left side of the funnel
                let text_x = segment_x_start as i32 - 120; // 120px to the left
                let text_y = segment_y_start as i32 + (segment_height as i32 / 2) + 5;
                (text_x, text_y)
            }
            crate::spec::ValueLabelPosition::Right => {
                // Position label on the right side of the funnel
                let text_x = segment_x_end as i32 + 10; // 10px to the right
                let text_y = segment_y_start as i32 + (segment_height as i32 / 2) + 5;
                (text_x, text_y)
            }
        };

        root.draw(&Text::new(
            label_text.as_str(),
            (text_x, text_y),
            style.axis_label_font(),
        ))
        .context("Failed to draw step label")?;
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
