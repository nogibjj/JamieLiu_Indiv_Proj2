use std::fs;
use std::path::Path;
use std::process::Command;

const TEST_DB: &str = "TestDrinksDB.db";

fn setup_test_db() {
    // Remove any existing test database
    if Path::new(TEST_DB).exists() {
        fs::remove_file(TEST_DB).expect("Failed to delete test database");
    }
    println!("Test database setup completed.");
}

fn teardown_test_db() {
    // Remove the test database after the test
    if Path::new(TEST_DB).exists() {
        fs::remove_file(TEST_DB).expect("Failed to delete test database");
    }
    println!("Test database teardown completed.");
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
fn test_read_data() {
    setup_test_db();

    // Load the dataset into the database
    let load_result = Command::new("cargo")
        .args(&["run", "--", "load"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute load command");
    assert!(load_result.status.success(), "Failed to load dataset");

    // Run the read-data command to print out the table
    let result = Command::new("cargo")
        .args(&["run", "--", "read-data"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute read_data command");

    // Print diagnostic information
    let stdout = String::from_utf8_lossy(&result.stdout);
    println!("Output from read-data:\n{}", stdout);

    // Check that the output contains known row identifiers (e.g., "Germany") to confirm table contents
    assert!(result.status.success());
    assert!(stdout.contains("Germany") && stdout.contains("USA"), "Expected rows not found in output");

    teardown_test_db();
}


#[test]
fn test_read_record() {
    setup_test_db();
    // Insert a test record for "Testland"
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
        .expect("Failed to insert test record for read_record");

    // Read the specific record for "Testland"
    let result = Command::new("cargo")
        .args(&["run", "--", "read-record", "Testland"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute read_record command");

    println!("Output from read-record: {:?}", result);
    let stdout = String::from_utf8_lossy(&result.stdout);
    println!("Standard output: {}", stdout);

    // Verify that the output contains "Testland"
    assert!(result.status.success());
    assert!(stdout.contains("Testland"));
    teardown_test_db();
}

#[test]
fn test_general_query() {
    setup_test_db();

    // Step 1: Load the data to populate the DrinksDB table
    let load_result = Command::new("cargo")
        .args(&["run", "--", "load"])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute load command");

    println!("Output from load: {:?}", load_result);
    assert!(load_result.status.success());

    // Step 2: Run a query to select only the Germany record
    let query_result = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "general-query",
            "SELECT * FROM DrinksDB WHERE country = 'Germany';",
        ])
        .env("DATABASE_URL", TEST_DB)
        .output()
        .expect("Failed to execute general_query command");

    let stdout = String::from_utf8_lossy(&query_result.stdout);
    println!("Output from general-query: {}", stdout);

    // Verify that the output contains "Germany"
    assert!(query_result.status.success(), "Query did not succeed");
    assert!(
        stdout.contains("Germany"),
        "Expected record for Germany not found in output"
    );

    teardown_test_db();
}
