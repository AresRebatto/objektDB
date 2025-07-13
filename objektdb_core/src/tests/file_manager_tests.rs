
use super::super::storage_engine::file_manager::*;
use std::fs;
use std::path::Path;

#[test]
fn test_create_db_success() {
    // Remove any previous test directory
    let db_name = "test_db_create";
    let _ = fs::remove_dir_all(db_name);

    // Should create the database successfully
    let result = create_db(db_name.to_string());
    assert!(result.is_ok());

    // Check if the .db file exists
    let db_file_path = format!("{}/{}.db", db_name, db_name);
    assert!(Path::new(&db_file_path).exists());

    // Clean up
    let _ = fs::remove_file(&db_file_path);
    let _ = fs::remove_dir(db_name);
}

#[test]
fn test_create_db_already_exists() {
    // Remove any previous test directory
    let db_name = "test_db_exists";
    let _ = fs::remove_dir_all(db_name);

    // First creation should succeed
    assert!(create_db(db_name.to_string()).is_ok());

    // Second creation should fail because db already exists
    let result = create_db(db_name.to_string());
    assert!(result.is_err());

    // Clean up
    let db_file_path = format!("{}/{}.db", db_name, db_name);
    let _ = fs::remove_file(&db_file_path);
    let _ = fs::remove_dir(db_name);
}

#[test]
fn test_create_db_invalid_dir() {
    // Try to create a database with an invalid directory name (Windows reserved name)
    let db_name = "con"; // "con" is reserved on Windows
    let result = create_db(db_name.to_string());
    assert!(result.is_err());
}