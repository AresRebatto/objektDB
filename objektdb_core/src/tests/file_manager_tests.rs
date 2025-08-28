use super::super::{storage_engine::file_manager::*, support_mods::{field::*, support_functions::*}};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

//create_db() tests
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

//create_table() tests
#[test]
fn test_create_table_success() {
    let db_name = "testdb";
    let table_name = "users";

    create_db(db_name.to_string()).expect("Failed to create database");

    let refs = vec!["roles".to_string()];
    let fields = vec![
        Field {
            name: "id".to_string(),
            is_OID: true
        },
        Field {
            name: "role_id".to_string(),
            is_OID: false
        },
    ];
    let methods = vec!["find_all".to_string()];

    let result = create_table(
        table_name.to_string(),
        db_name.to_string(),
        refs,
        fields,
        methods,
    );

    assert!(result.is_ok());

    let base = Path::new(db_name);
    assert!(base.join(format!("{}.tbl", table_name)).exists());
    assert!(base.join(format!("{}_bucket.bin", table_name)).exists());

    // Cleanup
    fs::remove_file(base.join(format!("{}.tbl", table_name))).unwrap();
    fs::remove_file(base.join(format!("{}_bucket.bin", table_name))).unwrap();
    fs::remove_file(base.join(format!("{}.db", db_name))).unwrap();
    fs::remove_dir_all(base).unwrap();
}

#[test]
fn test_create_table_missing_db() {
    let result = create_table(
        "some_table".to_string(),
        "nonexistent".to_string(),
        vec![],
        vec![],
        vec![],
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

#[test]
fn test_create_table_too_many_tables() {
    let db_name = "overflowdb";
    let table_name = "too_many";

    create_db(db_name.to_string()).unwrap();

    let db_path = Path::new(db_name).join(format!("{}.db", db_name));
    let mut content = fs::read(&db_path).unwrap();
    content[4] = 255; // simulate max tables reached
    fs::write(&db_path, &content).unwrap();

    let result = create_table(
        table_name.to_string(),
        db_name.to_string(),
        vec![],
        vec![],
        vec![],
    );

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Maximum number of tables reached (255)");

    // Cleanup
    fs::remove_file(db_path).unwrap();
    fs::remove_dir_all(db_name).unwrap();
}

#[test]
fn test_table_name_too_long() {
    let db_name = "longdb";
    let table_name = "a".repeat(65); // >64

    create_db(db_name.to_string()).unwrap();

    let result = create_table(
        table_name,
        db_name.to_string(),
        vec![],
        vec![],
        vec![],
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("too long"));

    // Cleanup
    fs::remove_file(Path::new(db_name).join(format!("{}.db", db_name))).unwrap();
    fs::remove_dir_all(db_name).unwrap();
}