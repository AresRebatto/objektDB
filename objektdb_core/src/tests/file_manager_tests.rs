use crate::storage_engine::file_manager::*;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

const TEST_DB_NAME: &str = "test_db";

#[test]
fn creates_new_database_file() {
    let path = format!("{}.db", TEST_DB_NAME);
    let _ = fs::remove_file(&path); // cleanup iniziale

    let result = create_db(TEST_DB_NAME.to_string());
    assert_eq!(result, Ok(()));

    assert!(Path::new(&path).exists(), "Database file should be created");

    let mut file = File::open(&path).expect("Failed to open created file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Failed to read file");

    assert_eq!(contents.len(), 10, "File should contain 10 bytes");

    assert_eq!(&contents[0..4], &MAGIC_NUMBER.to_le_bytes(), "MAGIC_NUMBER mismatch");
    assert_eq!(contents[4], 1u8, "Version byte mismatch");
    assert_eq!(contents[5], 0u8, "Number of tables should be 0");
    assert_eq!(&contents[6..10], &[0u8; 4], "Flags should be 4 zero bytes");

    let _ = fs::remove_file(&path);
}

#[test]
fn returns_error_if_file_exists() {
    let path = format!("{}.db", TEST_DB_NAME);
    let _ = File::create(&path); // crea file dummy

    let result = create_db(TEST_DB_NAME.to_string());
    assert!(result.is_err(), "Expected error when file already exists");

    let _ = fs::remove_file(&path);
}

#[test]
fn returns_error_with_invalid_filename() {
    // Solo su Windows il carattere "<" è illegale. Questo test può essere opzionale.
    #[cfg(windows)]
    {
        let result = create_db("invalid<name".to_string());
        assert!(result.is_err(), "Expected error with invalid filename");
    }
}