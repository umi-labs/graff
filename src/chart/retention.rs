use anyhow::{Context, Result};
use polars::prelude::*;
use plotters::prelude::*;
use crate::spec::{ChartConfig, LegendPosition};
use crate::render::styling::{get_chart_style, get_heatmap_style};

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
    // For retention charts, we need cohort_date, period_number, and users
    let cohort_date_col = config.cohort_date.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Retention charts require a 'cohort_date' field"))?;
    let period_number_col = config.period_number.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Retention charts require a 'period_number' field"))?;
    let users_col = config.users.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Retention charts require a 'users' field"))?;

    let cohort_col = df.column(cohort_date_col).context("Cohort date column not found")?;
    let period_col = df.column(period_number_col).context("Period number column not found")?;
    let users_data_col = df.column(users_col).context("Users column not found")?;

    // Collect retention data
    let mut retention_data: std::collections::HashMap<String, std::collections::HashMap<i32, f32>> = std::collections::HashMap::new();
    let mut all_cohorts = std::collections::HashSet::new();
    let mut all_periods = std::collections::HashSet::new();

    for i in 0..df.height().min(100) { // Limit for performance
        if let (Ok(cohort_val), Ok(period_val), Ok(users_val)) = (cohort_col.get(i), period_col.get(i), users_data_col.get(i)) {
            let cohort_str = format!("{:?}", cohort_val);
            let period_num = extract_numeric_value(period_val).unwrap_or(0.0) as i32;
            let users_count = extract_numeric_value(users_val).unwrap_or(0.0);
            
            retention_data.entry(cohort_str.clone()).or_default().insert(period_num, users_count);
            all_cohorts.insert(cohort_str);
            all_periods.insert(period_num);
        }
    }

    if retention_data.is_empty() {
        return Ok(());
    }

    // Convert to sorted vectors for consistent ordering
    let mut cohorts: Vec<String> = all_cohorts.into_iter().collect();
    cohorts.sort();
    let mut periods: Vec<i32> = all_periods.into_iter().collect();
    periods.sort();

    // Calculate retention percentages (normalize to first period = 100%)
    let mut retention_matrix = Vec::new();
    for cohort in &cohorts {
        let cohort_data = retention_data.get(cohort).unwrap();
        let mut cohort_retention = Vec::new();
        
        // Find the first period value (baseline)
        let baseline = periods.iter()
            .filter_map(|&p| cohort_data.get(&p))
            .next()
            .unwrap_or(&0.0);
        
        for &period in &periods {
            let value = cohort_data.get(&period).unwrap_or(&0.0);
            let retention_pct = if *baseline > 0.0 { (value / baseline) * 100.0 } else { 0.0 };
            cohort_retention.push(retention_pct);
        }
        
        retention_matrix.push(cohort_retention);
    }

    // Find max retention for scaling
    let max_retention = retention_matrix.iter()
        .flat_map(|row| row.iter())
        .fold(0.0f32, |max, &val| max.max(val));

    if max_retention == 0.0 {
        return Ok(());
    }

    let style = get_chart_style();
    let heatmap_style = get_heatmap_style();

    let mut chart = ChartBuilder::on(&root)
        .caption(title, style.title_font())
        .margin(style.layout.margins.chart as i32)
        .x_label_area_size(style.layout.areas.x_label_area)
        .y_label_area_size(style.layout.areas.y_label_area)
        .build_cartesian_2d(0.0f32..periods.len() as f32, 0.0f32..cohorts.len() as f32)
        .context("Failed to build chart")?;

    chart.configure_mesh()
        .x_desc("Period")
        .y_desc("Cohort")
        .axis_desc_style(style.axis_desc_font())
        .label_style(style.axis_label_font())
        .draw()
        .context("Failed to draw mesh")?;

    // Draw retention matrix cells
    for (cohort_idx, _cohort) in cohorts.iter().enumerate() {
        for (period_idx, &_period) in periods.iter().enumerate() {
            let retention_pct = retention_matrix[cohort_idx][period_idx];
            
            // Calculate color intensity based on retention percentage
            let intensity = retention_pct / max_retention;
            let base_color = heatmap_style.intensity_range.0 + 
                (intensity * (heatmap_style.intensity_range.1 - heatmap_style.intensity_range.0));
            let color = RGBColor(
                base_color as u8,
                (base_color * 0.8) as u8,
                (base_color * 0.6) as u8
            );

            // Draw retention cell
            chart
                .draw_series(std::iter::once(
                    Rectangle::new(
                        [(period_idx as f32, cohort_idx as f32), ((period_idx + 1) as f32, (cohort_idx + 1) as f32)],
                        color.filled()
                    )
                ))
                .context("Failed to draw retention cell")?;

            // Note: Retention percentages are shown via color intensity instead to avoid lifetime issues
        }
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