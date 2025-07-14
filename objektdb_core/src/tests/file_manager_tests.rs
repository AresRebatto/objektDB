use super::super::storage_engine::file_manager::*;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_create_db_success() {
    let db_name = "test_db_create";
    let db_dir_path = PathBuf::from(db_name);
    let db_file_path = db_dir_path.join(format!("{}.db", db_name));

    // Remove any previous test directory
    let _ = fs::remove_dir_all(&db_dir_path);

    // Should create the database successfully
    let result = create_db(db_name.to_string());
    assert!(result.is_ok(), "Database creation should succeed");

    // Check if the .db file exists
    assert!(db_file_path.exists(), "Database file was not created");

    // Clean up
    let _ = fs::remove_file(&db_file_path);
    let _ = fs::remove_dir(&db_dir_path);
}

#[test]
fn test_create_db_already_exists() {
    let db_name = "test_db_exists";
    let db_dir_path = PathBuf::from(db_name);
    let db_file_path = db_dir_path.join(format!("{}.db", db_name));

    // Remove any previous test directory
    let _ = fs::remove_dir_all(&db_dir_path);

    // First creation should succeed
    assert!(create_db(db_name.to_string()).is_ok());

    // Second creation should fail because the .db file already exists
    let result = create_db(db_name.to_string());
    assert!(result.is_err(), "Second database creation should fail");

    // Clean up
    let _ = fs::remove_file(&db_file_path);
    let _ = fs::remove_dir(&db_dir_path);
}

