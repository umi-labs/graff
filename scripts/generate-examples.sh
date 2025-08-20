#!/bin/bash

# Exit on any error
set -e

echo "🎨 Generating chart examples..."

# Create sample data files
echo "📊 Creating sample data..."

# Sample time series data
cat > sample_time_series.csv << EOF
date,totalUsers,channel
2024-01-01,1000,organic
2024-01-02,1200,organic
2024-01-03,1100,organic
2024-01-04,1400,organic
2024-01-05,1300,organic
2024-01-01,500,paid
2024-01-02,600,paid
2024-01-03,550,paid
2024-01-04,700,paid
2024-01-05,650,paid
EOF

# Sample categorical data
cat > sample_categorical.csv << EOF
category,value,segment
A,150,Group1
B,200,Group1
C,120,Group1
D,180,Group1
A,100,Group2
B,150,Group2
C,80,Group2
D,120,Group2
EOF

# Sample heatmap data
cat > sample_heatmap.csv << EOF
hour,weekday,sessions
0,Monday,50
1,Monday,30
2,Monday,20
3,Monday,15
4,Monday,10
5,Monday,25
6,Monday,45
7,Monday,80
8,Monday,120
9,Monday,150
10,Monday,180
11,Monday,200
12,Monday,220
13,Monday,210
14,Monday,190
15,Monday,170
16,Monday,160
17,Monday,140
18,Monday,130
19,Monday,110
20,Monday,90
21,Monday,70
22,Monday,60
23,Monday,55
0,Tuesday,45
1,Tuesday,25
2,Tuesday,15
3,Tuesday,10
4,Tuesday,8
5,Tuesday,20
6,Tuesday,40
7,Tuesday,75
8,Tuesday,110
9,Tuesday,140
10,Tuesday,170
11,Tuesday,190
12,Tuesday,210
13,Tuesday,200
14,Tuesday,180
15,Tuesday,160
16,Tuesday,150
17,Tuesday,130
18,Tuesday,120
19,Tuesday,100
20,Tuesday,80
21,Tuesday,65
22,Tuesday,55
23,Tuesday,50
EOF

# Sample funnel data
cat > sample_funnel.csv << EOF
step,value
Visited,10000
Viewed Product,5000
Added to Cart,2000
Started Checkout,1000
Completed Purchase,500
EOF

# Sample retention data
cat > sample_retention.csv << EOF
cohort_date,period_number,users
2024-01-01,0,1000
2024-01-01,1,800
2024-01-01,2,600
2024-01-01,3,400
2024-01-01,4,300
2024-01-08,0,1200
2024-01-08,1,900
2024-01-08,2,650
2024-01-08,3,450
2024-01-15,0,1100
2024-01-15,1,850
2024-01-15,2,620
2024-01-22,0,1300
2024-01-22,1,950
2024-01-29,0,1400
EOF

# Sample scatter data
cat > sample_scatter.csv << EOF
x_value,y_value,category
10,20,A
15,25,A
20,30,A
25,35,A
30,40,A
35,45,A
40,50,A
45,55,A
50,60,A
55,65,A
12,18,B
18,22,B
24,28,B
30,32,B
36,38,B
42,42,B
48,48,B
54,52,B
60,58,B
66,62,B
EOF

echo "✅ Sample data created"

# Function to generate chart with theme
generate_chart() {
    local chart_type=$1
    local output_name=$2
    local description=$3
    shift 3
    
    echo "📈 Generating $chart_type: $output_name"
    
    # Generate with light theme
    ./target/release/graff $chart_type "$@" --theme light --out "charts/$output_name-light.png"
    
    # Generate with dark theme
    ./target/release/graff $chart_type "$@" --theme dark --out "charts/$output_name-dark.png"
    
    echo "  ✅ Generated: $output_name (light & dark themes)"
}

# Line Charts
echo "📊 Generating Line Charts..."
generate_chart "line" "line-users-over-time" "Simple line chart showing user trends over time" \
    --input sample_time_series.csv --x date --y totalUsers --title "Users Over Time"

generate_chart "line" "line-users-by-channel" "Multi-series line chart showing users by channel" \
    --input sample_time_series.csv --x date --y totalUsers --group channel --title "Users by Channel"

# Area Charts
echo "📊 Generating Area Charts..."
generate_chart "area" "area-user-composition" "Stacked area chart showing user composition by channel" \
    --input sample_time_series.csv --x date --y totalUsers --group channel --title "User Composition Over Time"

generate_chart "area" "area-user-composition-percent" "Normalized area chart showing percentage composition" \
    --input sample_time_series.csv --x date --y totalUsers --group channel --normalize --title "User Composition (%)"

# Bar Charts
echo "📊 Generating Bar Charts..."
generate_chart "bar" "bar-values-by-category" "Simple bar chart showing values by category" \
    --input sample_categorical.csv --x category --y value --title "Values by Category"

generate_chart "bar" "bar-values-by-category-segment" "Grouped bar chart showing values by category and segment" \
    --input sample_categorical.csv --x category --y value --group segment --title "Values by Category and Segment"

generate_chart "bar" "bar-values-horizontal" "Horizontal bar chart for better label readability" \
    --input sample_categorical.csv --x category --y value --group segment --horizontal --title "Values by Category (Horizontal)"

# Stacked Bar Charts
echo "📊 Generating Stacked Bar Charts..."
generate_chart "bar-stacked" "bar-stacked-values" "Stacked bar chart showing composition by segment" \
    --input sample_categorical.csv --x category --y value --group segment --title "Stacked Values by Category"

# Heatmaps
echo "📊 Generating Heatmaps..."
generate_chart "heatmap" "heatmap-sessions-hour-day" "Heatmap showing session activity by hour and day of week" \
    --input sample_heatmap.csv --x hour --y weekday --z sessions --title "Sessions by Hour and Day"

generate_chart "heatmap" "heatmap-sessions-viridis" "Heatmap with viridis color scheme" \
    --input sample_heatmap.csv --x hour --y weekday --z sessions --colormap viridis --title "Sessions Heatmap (Viridis)"

# Scatter Plots
echo "📊 Generating Scatter Plots..."
generate_chart "scatter" "scatter-x-vs-y" "Simple scatter plot showing correlation between variables" \
    --input sample_scatter.csv --x x_value --y y_value --title "X vs Y Correlation"

generate_chart "scatter" "scatter-x-vs-y-by-category" "Scatter plot with points colored by category" \
    --input sample_scatter.csv --x x_value --y y_value --group category --title "X vs Y by Category"

# Funnel Charts
echo "📊 Generating Funnel Charts..."
generate_chart "funnel" "funnel-conversion" "Standard conversion funnel showing drop-off at each step" \
    --input sample_funnel.csv --steps "Visited,Viewed Product,Added to Cart,Started Checkout,Completed Purchase" --values value --step-order "0,1,2,3,4" --title "Conversion Funnel"

generate_chart "funnel" "funnel-conversion-left-labels" "Funnel chart with value labels on the left side" \
    --input sample_funnel.csv --steps "Visited,Viewed Product,Added to Cart,Started Checkout,Completed Purchase" --values value --value-labels left --step-order "0,1,2,3,4" --title "Conversion Funnel (Left Labels)"

# Retention Charts
echo "📊 Generating Retention Charts..."
generate_chart "retention" "retention-matrix" "Retention matrix showing user retention over time" \
    --input sample_retention.csv --cohort-date cohort_date --period-number period_number --users users --title "User Retention Matrix"

generate_chart "retention" "retention-matrix-percent" "Retention matrix showing retention percentages" \
    --input sample_retention.csv --cohort-date cohort_date --period-number period_number --users users --percentage --title "User Retention Matrix (%)"

echo "🎉 All chart examples generated successfully!"

# Clean up sample data files
rm sample_time_series.csv sample_categorical.csv sample_heatmap.csv sample_funnel.csv sample_retention.csv sample_scatter.csv

echo "🧹 Sample data files cleaned up"
