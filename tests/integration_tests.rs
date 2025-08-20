use std::fs;
use std::path::Path;
use tempfile::TempDir;

// Helper function to create a temporary test directory
fn create_test_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

// Helper function to create a test CSV file
fn create_test_csv(dir: &Path, filename: &str, content: &str) {
    let file_path = dir.join(filename);
    fs::write(file_path, content).expect("Failed to write test CSV");
}

// Helper function to create a test YAML spec file
fn create_test_spec(dir: &Path, filename: &str, content: &str) {
    let file_path = dir.join(filename);
    fs::write(file_path, content).expect("Failed to write test spec");
}

#[test]
fn test_cli_line_chart_basic() {
    let test_dir = create_test_dir();
    let csv_content = "date,users,channel\n2023-01-01,100,organic\n2023-01-02,150,direct";
    create_test_csv(test_dir.path(), "test.csv", csv_content);
    
    let output_path = test_dir.path().join("output.png");
    
    // Test the line chart command
    let result = std::process::Command::new("cargo")
        .args([
            "run", "--", "line",
            "--input", test_dir.path().join("test.csv").to_str().unwrap(),
            "--x", "date",
            "--y", "users",
            "--out", output_path.to_str().unwrap(),
        ])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Check if the command succeeded
    if !output.status.success() {
        println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success(), "CLI command failed");
    
    // Check if output file was created
    assert!(output_path.exists(), "Output file was not created");
    assert!(output_path.metadata().unwrap().len() > 0, "Output file is empty");
}

#[test]
fn test_cli_bar_chart_basic() {
    let test_dir = create_test_dir();
    let csv_content = "category,value\nA,100\nB,200\nC,150";
    create_test_csv(test_dir.path(), "test.csv", csv_content);
    
    let output_path = test_dir.path().join("output.png");
    
    // Test the bar chart command
    let result = std::process::Command::new("cargo")
        .args([
            "run", "--", "bar",
            "--input", test_dir.path().join("test.csv").to_str().unwrap(),
            "--x", "category",
            "--y", "value",
            "--out", output_path.to_str().unwrap(),
        ])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    if !output.status.success() {
        println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success(), "CLI command failed");
    assert!(output_path.exists(), "Output file was not created");
}

#[test]
fn test_cli_render_spec_file() {
    let test_dir = create_test_dir();
    
    // Create test CSV
    let csv_content = "date,users,channel\n2023-01-01,100,organic\n2023-01-02,150,direct";
    create_test_csv(test_dir.path(), "test.csv", csv_content);
    
    // Create test spec file with absolute paths
    let spec_content = format!(
        r#"
charts:
  - type: line
    title: "Test Line Chart"
    data: "{}"
    x: "date"
    y: "users"
  - type: bar
    title: "Test Bar Chart"
    data: "{}"
    x: "channel"
    y: "users"
"#,
        test_dir.path().join("test.csv").to_str().unwrap(),
        test_dir.path().join("test.csv").to_str().unwrap()
    );
    create_test_spec(test_dir.path(), "test_spec.yaml", &spec_content);
    
    // Test the render command - run from the project root, not the temp dir
    let result = std::process::Command::new("cargo")
        .args([
            "run", "--", "render",
            "--spec", test_dir.path().join("test_spec.yaml").to_str().unwrap(),
            "--out", test_dir.path().to_str().unwrap(),
        ])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    if !output.status.success() {
        println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success(), "CLI command failed");
    
    // Check if output files were created
    let output_files: Vec<_> = fs::read_dir(test_dir.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "png" || ext == "svg")
                .unwrap_or(false)
        })
        .collect();
    
    assert!(!output_files.is_empty(), "No output files were created");
}

#[test]
fn test_cli_error_handling_missing_file() {
    // Test error handling for missing input file
    let result = std::process::Command::new("cargo")
        .args([
            "run", "--", "line",
            "--input", "nonexistent.csv",
            "--x", "date",
            "--y", "users",
            "--out", "output.png",
        ])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Should fail with a meaningful error message
    assert!(!output.status.success(), "Command should fail for missing file");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Failed to open CSV file") || stderr.contains("not found"),
        "Error message should mention file issue: {}",
        stderr
    );
}

#[test]
fn test_cli_error_handling_missing_columns() {
    let test_dir = create_test_dir();
    let csv_content = "date,users\n2023-01-01,100\n2023-01-02,150";
    create_test_csv(test_dir.path(), "test.csv", csv_content);
    
    // Test error handling for missing column
    let result = std::process::Command::new("cargo")
        .args([
            "run", "--", "line",
            "--input", test_dir.path().join("test.csv").to_str().unwrap(),
            "--x", "date",
            "--y", "nonexistent_column",
            "--out", test_dir.path().join("output.png").to_str().unwrap(),
        ])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    // Should fail with a meaningful error message
    assert!(!output.status.success(), "Command should fail for missing column");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Column") && stderr.contains("not found"),
        "Error message should mention missing column: {}",
        stderr
    );
}

#[test]
fn test_cli_help_output() {
    // Test that help output is comprehensive
    let result = std::process::Command::new("cargo")
        .args(["run", "--", "--help"])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    assert!(output.status.success(), "Help command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for key elements in help output
    assert!(stdout.contains("graff"), "Help should mention graff");
    assert!(stdout.contains("Commands:"), "Help should list commands");
    assert!(stdout.contains("line"), "Help should mention line command");
    assert!(stdout.contains("bar"), "Help should mention bar command");
    assert!(stdout.contains("render"), "Help should mention render command");
}

#[test]
fn test_cli_line_chart_help() {
    // Test that line chart help is comprehensive
    let result = std::process::Command::new("cargo")
        .args(["run", "--", "line", "--help"])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    assert!(output.status.success(), "Line help command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for key elements in line chart help
    assert!(stdout.contains("line"), "Help should mention line");
    assert!(stdout.contains("--input"), "Help should mention --input");
    assert!(stdout.contains("--x"), "Help should mention --x");
    assert!(stdout.contains("--y"), "Help should mention --y");
    assert!(stdout.contains("--out"), "Help should mention --out");
}

#[test]
fn test_cli_with_theme_option() {
    let test_dir = create_test_dir();
    let csv_content = "date,users\n2023-01-01,100\n2023-01-02,150";
    create_test_csv(test_dir.path(), "test.csv", csv_content);
    
    let output_path = test_dir.path().join("output.png");
    
    // Test the line chart command with dark theme
    let result = std::process::Command::new("cargo")
        .args([
            "run", "--", "line",
            "--input", test_dir.path().join("test.csv").to_str().unwrap(),
            "--x", "date",
            "--y", "users",
            "--out", output_path.to_str().unwrap(),
            "--theme", "dark",
        ])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    if !output.status.success() {
        println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success(), "CLI command with theme should succeed");
    assert!(output_path.exists(), "Output file was not created");
}

#[test]
fn test_cli_with_scale_option() {
    let test_dir = create_test_dir();
    let csv_content = "date,users\n2023-01-01,100\n2023-01-02,150";
    create_test_csv(test_dir.path(), "test.csv", csv_content);
    
    let output_path = test_dir.path().join("output.png");
    
    // Test the line chart command with scale factor
    let result = std::process::Command::new("cargo")
        .args([
            "run", "--", "line",
            "--input", test_dir.path().join("test.csv").to_str().unwrap(),
            "--x", "date",
            "--y", "users",
            "--out", output_path.to_str().unwrap(),
            "--scale", "2.0",
        ])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    if !output.status.success() {
        println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success(), "CLI command with scale should succeed");
    assert!(output_path.exists(), "Output file was not created");
}

#[test]
fn test_cli_with_format_option() {
    let test_dir = create_test_dir();
    let csv_content = "date,users\n2023-01-01,100\n2023-01-02,150";
    create_test_csv(test_dir.path(), "test.csv", csv_content);
    
    let output_path = test_dir.path().join("output.png");
    
    // Test the line chart command with PNG format (SVG might not be fully supported)
    let result = std::process::Command::new("cargo")
        .args([
            "run", "--", "line",
            "--input", test_dir.path().join("test.csv").to_str().unwrap(),
            "--x", "date",
            "--y", "users",
            "--out", output_path.to_str().unwrap(),
            "--format", "png",
        ])
        .output();
    
    assert!(result.is_ok());
    let output = result.unwrap();
    
    if !output.status.success() {
        println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    assert!(output.status.success(), "CLI command with PNG format should succeed");
    assert!(output_path.exists(), "PNG output file was not created");
    
    // Check that it's actually a PNG file (should have PNG header)
    let content = fs::read(&output_path).unwrap();
    assert!(content.len() > 8, "PNG file should have content");
    // PNG files start with the magic bytes: 89 50 4E 47 0D 0A 1A 0A
    assert_eq!(&content[0..8], [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], "File should be a valid PNG");
}
