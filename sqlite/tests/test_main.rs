use std::fs;
use std::path::Path;
use std::process::Command;

const TEST_DB: &str = "TestDrinksDB.db";

fn setup_test_db() {
    // Remove any existing test database
    if Path::new(TEST_DB).exists() {
        fs::remove_file(TEST_DB).expect("Failed to delete test database");
    }
}

fn teardown_test_db() {
    // Remove the test database after the test
    if Path::new(TEST_DB).exists() {
        fs::remove_file(TEST_DB).expect("Failed to delete test database");
    }
}

#[test]
fn test_extract() {
    setup_test_db();
    let result = Command::new("cargo")
        .args(&["run", "--", "extract"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute extract command");

    println!("Output from extract: {:?}", result);
    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Extracting data..."));
    teardown_test_db();
}

#[test]
fn test_load() {
    setup_test_db();
    let result = Command::new("cargo")
        .args(&["run", "--", "load"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute load command");

    println!("Output from load: {:?}", result);
    assert!(result.status.success());
    let stdout = String::from_utf8_lossy(&result.stdout);
    assert!(stdout.contains("Transforming and loading data..."));
    teardown_test_db();
}

#[test]
fn test_create_record() {
    setup_test_db();
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
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute create_record command");

    println!("Output from create-record: {:?}", result);
    assert!(result.status.success());
    teardown_test_db();
}

#[test]
fn test_update_record() {
    setup_test_db();
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
        .env("DATABASE_URL", TEST_DB)
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
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute update_record command");

    println!("Output from update-record: {:?}", result);
    assert!(result.status.success());
    teardown_test_db();
}

#[test]
fn test_delete_record() {
    setup_test_db();
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
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to insert test record for delete_record");

    let result = Command::new("cargo")
        .args(&["run", "--", "delete-record", "Testland"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute delete_record command");

    println!("Output from delete-record: {:?}", result);
    assert!(result.status.success());
    teardown_test_db();
}

#[test]
fn test_general_query() {
    setup_test_db();

    // Step 1: Create the record
    let create_result = Command::new("cargo")
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
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to insert test record for general_query");

    println!("Output from create-record: {:?}", create_result);
    assert!(create_result.status.success());

    // Step 2: Check if the record was created successfully
    let check_result = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "general-query",
            "SELECT country FROM DrinksDB WHERE country = 'Testland'",
        ])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute general_query command");

    println!("Output from general-query: {:?}", check_result);
    let stdout = String::from_utf8_lossy(&check_result.stdout);
    println!("Standard output: {}", stdout); // Print stdout for further inspection

    assert!(check_result.status.success());
    assert!(stdout.contains("Testland"), "Record not found in database");

    teardown_test_db();
}

#[test]
fn test_read_data() {
    setup_test_db();
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
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to insert test record for read_data");

    let result = Command::new("cargo")
        .args(&["run", "--", "read-data"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute read_data command");

    println!("Output from read-data: {:?}", result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    println!("Standard output: {}", stdout);

    assert!(result.status.success());
    assert!(stdout.contains("Testland"));
    teardown_test_db();
}
