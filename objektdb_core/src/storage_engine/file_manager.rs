use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Magic number that identifies the custom database file format.
///
/// This constant is written at the beginning of every `.db` file created by the system.
/// It serves as a unique identifier to recognize and validate binary files that conform
/// to this format.
///
/// The magic number is encoded as a `u32` in **little-endian** byte order.
///
/// # Purpose
/// - Verifies file integrity.
/// - Detects corrupted or incompatible files.
/// - Rejects files that do not match the expected format.
pub const MAGIC_NUMBER: u32 = 0x4D594442;

///Creates a new database from the name. 
/// 
/// It initially creates it empty, and you can then insert tables using the 
/// `create_table()` method.
///The binary file that is created by the method follows the following format:
/// ```json
///DATABASE_HEADER {
///    magic_number = 0x4D594442, (4 byte(u32))
///    version = 1, (1 byte(u8))
///    num_tablespaces = N, (1 byte(u8))
///    flags = u32   (4 byte)
///}
///
///TABLESPACE_DIRECTORY[N] {
///    TablespaceEntry {
///        name = "User", (64 byte)
///        file_path = "user.tbl", (68 byte)
///        offset = 123, (4 byte(u32))
///        checksum = u32, (4 byte(u32))
///        last_oid = u64,  (8 byte(u64))
///    }
///    ...
///}
/// ```
/// # Arguments
///
/// * `db_name` - The name of the database (without the `.db` extension).
///
/// # Returns
///
/// * `Ok(())` if the database file was successfully created.
/// * `Err(String)` if the file does not exist or if an error occurred during creation.
/// # Example
/// ```
/// use objektoDB::storage_engine::file_manager::create_db;
/// 
/// match create_db(String::from("my_database")) {
///     Ok(_) => println!("Database created successfully!"),
///     Err(e) => println!("Error creating database: {}", e),
/// }
///If the conversion is successful, it returns `Ok(())`, otherwise `Err(error)`
pub fn create_db(db_name : String) -> Result<(), String> {
    let db_path = format!("{}.db", _name);

    if !Path::new(&db_path).exists() {

        match OpenOptions::new().write(true).create_new(true).open(&db_path) {
            Ok(mut f) => {
                let mut buffer = Vec::new();

                //header
                buffer.extend_from_slice(&MAGIC_NUMBER.to_le_bytes());
                buffer.extend_from_slice(&[1 as u8]); //Version
                buffer.extend_from_slice(&[0u8; 1]); //Number of tables
                buffer.extend_from_slice(&[0u8; 4]); //flags(for future use)

                f.write_all(&buffer ).map_err(|e| format!("Error writing to the file: {}", e))?;
                return Ok(());
            },
            Err(e) => return Err(format!("Error in the file creation: {}", e)),
        }


    } else {
        return Err(format!("Database {} already exists", _name));
    }   

    
}   

pub fn create_table(_table_name: String, _db_name: String) -> Result<(), String> {
    todo!();
}


/// Deletes the specified database file from the filesystem.
///
/// This function attempts to remove the database file with the given name.
/// The database file is expected to have a `.db` extension. If the file exists,
/// it will be deleted. If the file does not exist, an error is returned.
///
/// # Arguments
///
/// * `db_name` - The name of the database (without the `.db` extension) to delete.
///
/// # Returns
///
/// * `Ok(())` if the database file was successfully deleted.
/// * `Err(String)` if the file does not exist or if an error occurred during deletion.
///
/// # Example
///
/// ```
/// use objektoDB::storage_engine::file_manager::delete_db;
///
/// match delete_db(String::from("my_database")) {
///     Ok(_) => println!("Database deleted successfully!"),
///     Err(e) => println!("Error deleting database: {}", e),
/// }
/// ```
pub fn delete_db(db_name: String) -> Result<(), String> {
    let db_name = input.to_string();
    let db_path = format!("{}.db", db_name);
    
    if Path::new(&db_path).exists() {
        std::fs::remove_file(&db_path).map_err(|e| format!("Error deleting database: {}", e))?;
        Ok(())
    } else {
        Err(format!("Database {} does not exist", db_name))
    }
}

