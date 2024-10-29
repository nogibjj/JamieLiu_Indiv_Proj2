use std::process::Command;

#[test]
fn test_extract() {
    let result = Command::new("cargo")
        .args(&["run", "--", "extract"])
        .output()
        .expect("Failed to execute extract command");

    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Extracting data..."));
}

#[test]
fn test_load() {
    let result = Command::new("cargo")
        .args(&["run", "--", "load"])
        .output()
        .expect("Failed to execute load command");

    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Transforming and loading data..."));
}

#[test]
fn test_create_record() {
    let result = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "create-record",
            "Testland",
            "300",
            "200",
            "150",
            "12.5",
        ])
        .output()
        .expect("Failed to execute create_record command");

    assert!(result.status.success());
}

#[test]
fn test_update_record() {
    // Ensure "Testland" exists by creating it before updating
    let _ = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "create-record",
            "Testland",
            "300",
            "200",
            "150",
            "12.5",
        ])
        .output()
        .expect("Failed to insert test record for update_record");

    let result = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "update-record",
            "Testland",
            "320",
            "210",
            "160",
            "13.0",
        ])
        .output()
        .expect("Failed to execute update_record command");

    assert!(result.status.success());
}

#[test]
fn test_delete_record() {
    // Ensure "Testland" exists by creating it before deletion
    let _ = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "create-record",
            "Testland",
            "300",
            "200",
            "150",
            "12.5",
        ])
        .output()
        .expect("Failed to insert test record for delete_record");

    let result = Command::new("cargo")
        .args(&["run", "--", "delete-record", "Testland"])
        .output()
        .expect("Failed to execute delete_record command");

    assert!(result.status.success());
}

#[test]
fn test_general_query() {
    // Ensure "Testland" exists by creating it before querying
    let _ = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "create-record",
            "Testland",
            "300",
            "200",
            "150",
            "12.5",
        ])
        .output()
        .expect("Failed to insert test record for general_query");

    let result = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "general-query",
            "SELECT * FROM DrinksDB WHERE country = 'Testland'",
        ])
        .output()
        .expect("Failed to execute general_query command");

    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Testland"));
}

#[test]
fn test_read_data() {
    // Ensure "Testland" exists by creating it before reading
    let _ = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "create-record",
            "Testland",
            "300",
            "200",
            "150",
            "12.5",
        ])
        .output()
        .expect("Failed to insert test record for read_data");

    let result = Command::new("cargo")
        .args(&["run", "--", "read-data"])
        .output()
        .expect("Failed to execute read_data command");

    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Testland"));
}
