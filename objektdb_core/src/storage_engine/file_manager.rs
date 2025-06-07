use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{self, Path};

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
///The header of the binary file that is created by the method follows the following format:
/// ```json
///DATABASE_HEADER {
///    magic_number, (4 byte)
///    version, (1 byte)
///    num_tablespaces, (1 byte)
///    flags (4 byte)
///}
///
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
    let db_path = format!("{}.db", db_name);

    if !Path::new(&db_path).exists() {

        match File::create(&db_path) {
            Ok(mut f) => {
            let mut buffer = Vec::with_capacity(10);

            //header
            buffer.extend_from_slice(&MAGIC_NUMBER.to_le_bytes());// Magic number (4 byte)
            buffer.extend_from_slice(&[1u8]); // Version(1 byte)
            buffer.extend_from_slice(&[0u8; 1]); // Number of tables(1 byte)
            buffer.extend_from_slice(&[0u8; 4]); // flags (for future use)(4 byte)

            f.write_all(&buffer).map_err(|e| format!("Error writing to the file: {}", e))?;
            return Ok(());
            },
            Err(e) => return Err(format!("Error in the file creation: {}", e)),
        }


    } else {
        return Err(format!("Database {} already exists", db_name));
    }   

    
}   

/// Creates a new table in the specified database.
/// 
/// This function initializes a new table within an existing database file.
/// It checks if the database file exists and is in the correct format: in case
/// it is not, it returns an error.
/// Before creating the table, please ensure that the database file has been created using
/// the `create_db()` function.
/// One table has the following format:
/// ```json
/// TablespaceEntry {
///        name, (64 byte)
///        file_path, (68 byte-> 64 byte + 4 byte for extension "tbl")
///        offset, (4 byte)
///        checksum, (4 byte)
///        last_oid (8 byte)        
///    }
/// ```
/// # Arguments
/// * `_table_name` - The name of the table to be created.
/// * `_db_name` - The name of the database where the table will be created (without the `.db` extension).
/// # Returns
/// * `Ok(())` if the table was successfully created.
/// * `Err(String)` if the database file does not exist, is invalid, or if an error occurred during the process.
/// # Example
/// ```
/// use objektoDB::storage_engine::file_manager::create_table;
/// match create_table(String::from("my_table"), String::from("my_database")) {
///    Ok(_) => println!("Table created successfully!"),
///    Err(e) => println!("Error creating table: {}", e),
/// }
/// ```
/// 
pub fn create_table(_table_name: String, _db_name: String) -> Result<(), String> {

    let path = format!("{}.db", _db_name);

    // Check if the database file exists
    if Path::new(&path).exists() {

        let mut file = File::open(&path).map_err(|e| format!("Error opening database file: {}", e))?;
        let mut buffer = Vec::new();

        file.read(&mut buffer).map_err(|e| format!("Error reading database file: {}", e))?;

        if buffer[0..4] == MAGIC_NUMBER.to_le_bytes() {
            todo!("Implement table creation logic here");

            // header len + table num * one table len
            let offset = 10 + buffer[4] * 148;
            //let table_offset = buffer[16..23];
        }else{
            return Err("Invalid database file format".to_string());
        }
    }else{
        return Err(format!("Database {} does not exist", _db_name));
    }
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
    let db_name = db_name;
    let db_path = format!("{}.db", db_name);
    
    if Path::new(&db_path).exists() {
        std::fs::remove_file(&db_path).map_err(|e| format!("Error deleting database: {}", e))?;
        Ok(())
    } else {
        Err(format!("Database {} does not exist", db_name))
    }
}

