use crate::spec::Theme;
use plotters::prelude::*;

/// Centralized styling configuration for all chart types
pub struct ChartStyle {
    pub colors: ColorPalette,
    pub typography: Typography,
    pub layout: Layout,
    pub spacing: Spacing,
    pub theme: Theme,
}

impl Default for ChartStyle {
    fn default() -> Self {
        Self::new(Theme::Light)
    }
}

impl ChartStyle {
    /// Create a new chart style with the specified theme
    pub fn new(theme: Theme) -> Self {
        Self {
            colors: ColorPalette::new(&theme),
            typography: Typography::default(),
            layout: Layout::default(),
            spacing: Spacing::default(),
            theme,
        }
    }
}

/// Color palette for charts
pub struct ColorPalette {
    /// Primary colors for data series (bars, lines, etc.)
    pub primary: Vec<RGBColor>,
    /// Text colors
    pub text: TextColors,
    /// Background and grid colors
    pub background: BackgroundColors,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::new(&Theme::Light)
    }
}

impl ColorPalette {
    /// Create a new color palette with the specified theme
    pub fn new(theme: &Theme) -> Self {
        match theme {
            Theme::Light => Self::light(),
            Theme::Dark => Self::dark(),
        }
    }

    fn light() -> Self {
        Self {
            // Colorblind-friendly palette based on ColorBrewer
            primary: vec![
                RGBColor(31, 119, 180),  // Blue
                RGBColor(255, 127, 14),  // Orange
                RGBColor(44, 160, 44),   // Green
                RGBColor(214, 39, 40),   // Red
                RGBColor(148, 103, 189), // Purple
                RGBColor(140, 86, 75),   // Brown
                RGBColor(227, 119, 194), // Pink
                RGBColor(127, 127, 127), // Gray
                RGBColor(188, 189, 34),  // Olive
                RGBColor(23, 190, 207),  // Cyan
            ],
            text: TextColors::light(),
            background: BackgroundColors::light(),
        }
    }

    fn dark() -> Self {
        Self {
            // Same colorblind-friendly palette, adjusted for dark theme
            primary: vec![
                RGBColor(114, 158, 206), // Lighter blue
                RGBColor(255, 158, 74),  // Lighter orange
                RGBColor(103, 191, 92),  // Lighter green
                RGBColor(237, 102, 93),  // Lighter red
                RGBColor(173, 139, 201), // Lighter purple
                RGBColor(168, 120, 110), // Lighter brown
                RGBColor(237, 151, 202), // Lighter pink
                RGBColor(162, 162, 162), // Lighter gray
                RGBColor(205, 204, 93),  // Lighter olive
                RGBColor(109, 204, 218), // Lighter cyan
            ],
            text: TextColors::dark(),
            background: BackgroundColors::dark(),
        }
    }
}

pub struct TextColors {
    /// Main title color
    pub title: RGBColor,
    /// Axis labels and descriptions
    pub axis_labels: RGBColor,
    /// Data point labels and legends
    pub data_labels: RGBColor,
    /// Grid and mesh lines
    pub grid: RGBColor,
}

impl Default for TextColors {
    fn default() -> Self {
        Self::light()
    }
}

impl TextColors {
    fn light() -> Self {
        Self {
            title: RGBColor(33, 37, 41),          // Dark gray for titles
            axis_labels: RGBColor(80, 80, 80),    // Medium gray for axis
            data_labels: RGBColor(100, 100, 100), // Light gray for data
            grid: RGBColor(222, 226, 230),        // Very light gray for grid
        }
    }

    fn dark() -> Self {
        Self {
            title: RGBColor(248, 249, 250),       // Light gray for titles
            axis_labels: RGBColor(200, 200, 200), // Medium gray for axis
            data_labels: RGBColor(180, 180, 180), // Light gray for data
            grid: RGBColor(73, 80, 87),           // Dark gray for grid
        }
    }
}

pub struct BackgroundColors {
    /// Chart background
    pub chart: RGBColor,
    /// Canvas background
    pub canvas: RGBColor,
}

impl Default for BackgroundColors {
    fn default() -> Self {
        Self::light()
    }
}

impl BackgroundColors {
    fn light() -> Self {
        Self {
            chart: WHITE,
            canvas: WHITE,
        }
    }

    fn dark() -> Self {
        Self {
            chart: RGBColor(33, 37, 41),  // Dark gray
            canvas: RGBColor(33, 37, 41), // Dark gray
        }
    }
}

/// Typography settings
pub struct Typography {
    /// Font family (same for all text)
    pub font_family: &'static str,
    /// Font sizes for different elements
    pub sizes: FontSizes,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "sans-serif",
            sizes: FontSizes::default(),
        }
    }
}

pub struct FontSizes {
    /// Main chart title
    pub title: u32,
    /// Axis descriptions (x-axis, y-axis labels)
    pub axis_description: u32,
    /// Axis tick labels (numbers, categories)
    pub axis_labels: u32,
    /// Legend text
    pub legend: u32,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            title: 36,
            axis_description: 20,
            axis_labels: 16,
            legend: 14,
        }
    }
}

/// Layout and sizing configuration
pub struct Layout {
    /// Chart margins
    pub margins: Margins,
    /// Area sizes for different chart elements
    pub areas: AreaSizes,
    /// Point and line sizes
    pub elements: ElementSizes,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            margins: Margins::default(),
            areas: AreaSizes::default(),
            elements: ElementSizes::default(),
        }
    }
}

pub struct Margins {
    /// General margin around chart
    pub chart: u32,
    /// Extra margin for complex charts
    pub complex: u32,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            chart: 30,
            complex: 40,
        }
    }
}

pub struct AreaSizes {
    /// X-axis label area height
    pub x_label_area: u32,
    /// Y-axis label area width
    pub y_label_area: u32,
    /// Legend area size
    pub legend_area: u32,
}

impl Default for AreaSizes {
    fn default() -> Self {
        Self {
            x_label_area: 80,
            y_label_area: 80,
            legend_area: 60,
        }
    }
}

pub struct ElementSizes {
    /// Line chart point size
    pub line_points: u32,
    /// Line width for line charts
    pub line_width: u32,
    /// Bar spacing factor
    pub bar_spacing: f32,
}

impl Default for ElementSizes {
    fn default() -> Self {
        Self {
            line_points: 4,
            line_width: 2,
            bar_spacing: 0.8,
        }
    }
}

/// Spacing and padding configuration
pub struct Spacing {
    /// Padding between chart elements
    pub element_padding: u32,
    /// Spacing between data series
    pub series_spacing: u32,
    /// Padding for text elements
    pub text_padding: u32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            element_padding: 10,
            series_spacing: 5,
            text_padding: 8,
        }
    }
}

/// Heatmap-specific styling
pub struct HeatmapStyle {
    /// Color intensity range for heatmaps
    pub intensity_range: (f32, f32),
    /// Base colors for heatmap gradients
    pub gradient_colors: (RGBColor, RGBColor),
}

impl Default for HeatmapStyle {
    fn default() -> Self {
        Self {
            intensity_range: (60.0, 180.0), // Light gray to darker blue-gray
            gradient_colors: (
                RGBColor(180, 190, 200), // Light blue-gray
                RGBColor(60, 80, 120),   // Dark blue-gray
            ),
        }
    }
}

/// Helper functions for creating styled fonts and colors
impl ChartStyle {
    /// Get a primary color by index (cycles through available colors)
    pub fn get_primary_color(&self, index: usize) -> &RGBColor {
        &self.colors.primary[index % self.colors.primary.len()]
    }

    /// Create a title font style
    pub fn title_font(&self) -> TextStyle<'_> {
        (self.typography.font_family, self.typography.sizes.title)
            .into_font()
            .color(&self.colors.text.title)
    }

    /// Create an axis description font style
    pub fn axis_desc_font(&self) -> TextStyle<'_> {
        (
            self.typography.font_family,
            self.typography.sizes.axis_description,
        )
            .into_font()
            .color(&self.colors.text.axis_labels)
    }

    /// Create an axis label font style
    pub fn axis_label_font(&self) -> TextStyle<'_> {
        (
            self.typography.font_family,
            self.typography.sizes.axis_labels,
        )
            .into_font()
            .color(&self.colors.text.data_labels)
    }

    /// Create a legend font style
    pub fn legend_font(&self) -> TextStyle<'_> {
        (self.typography.font_family, self.typography.sizes.legend)
            .into_font()
            .color(&self.colors.text.data_labels)
    }
}

/// Global style instance
pub fn get_chart_style() -> ChartStyle {
    ChartStyle::default()
}

/// Get chart style with specific theme
pub fn get_chart_style_with_theme(theme: &Theme) -> ChartStyle {
    ChartStyle::new(theme.clone())
}

/// Heatmap-specific styling
pub fn get_heatmap_style() -> HeatmapStyle {
    HeatmapStyle::default()
}
