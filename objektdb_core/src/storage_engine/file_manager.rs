use std::f32::consts::E;
use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{self, Path, PathBuf};

//DA RIVEDERE TUTTA LA DOCUMENTAZIONE IN TESTA AI METODI


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

/// Creates a new database from the name. 
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
/// use objektDB::storage_engine::file_manager::create_db;
/// 
/// match create_db(String::from("my_database")) {
///     Ok(_) => println!("Database created successfully!"),
///     Err(e) => println!("Error creating database: {}", e),
/// }
/// ```
///If the conversion is successful, it returns `Ok(())`, otherwise `Err(error)`
pub fn create_db(db_name : String) -> Result<(), String>{
    match std::fs::create_dir(&db_name){
        Err(e)=> Err(format!("Error creating the database directory: {}", e)),
        Ok(_)=>{

            //we need to do binding for the life time
            let path_dir = format!("{}/{}.db", db_name, db_name);
            let db_path = Path::new(&path_dir);
            if db_path.exists(){
                return Err("The database already exists".to_string()); 
            }

            
            match File::create(db_path){
                Err(e) => {
                    Err(format!("Error creating db: {}", e))

                    //In caso di annullamento, si deve vedere se esiste ancora
                    //la directory e, in caso, eliminarla per abortire l'operazione
                },
                Ok(_)=>{
                    return Ok(());
                }
            }
            
            
        }
    }
}
/*pub fn create_db(db_name : String) -> Result<(), String> {
    if let Err(e) = std::fs::create_dir(&db_name) {
        return Err(format!("Error creating database directory: {}", e));
    }
    let db_dir = Path::new(&db_name);
    let db_path = db_dir.join(format!("{}.db", db_name));

    if db_path.exists() {
        return Err(format!("Database {} already exists", db_name));
    }

    match File::create(&db_path) {
        Ok(mut f) => {
            let mut buffer = Vec::with_capacity(10);

            // header
            buffer.extend_from_slice(&MAGIC_NUMBER.to_le_bytes()); // Magic number (4 byte)
            buffer.extend_from_slice(&[1u8]); // Version (1 byte)
            buffer.extend_from_slice(&[0u8; 1]); // Number of tables (1 byte)
            buffer.extend_from_slice(&[0u8; 4]); // flags (for future use) (4 byte)

            f.write_all(&buffer).map_err(|e| format!("Error writing to the file: {}", e))?;
            Ok(())
        },
        Err(e) => Err(format!("Error in the file creation: {}", e)),
    }
}*/

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
/// use objektDB::storage_engine::file_manager::create_table;
/// match create_table(String::from("my_table"), String::from("my_database")) {
///    Ok(_) => println!("Table created successfully!"),
///    Err(e) => println!("Error creating table: {}", e),
/// }
/// ```
/// 
pub fn create_table(_table_name: String, _db_name: String) -> Result<(), String> {

    let path = format!("{}/{}.db", _db_name, _db_name);

    // Check if the database file exists
    if Path::new(&path).exists() {

        let mut file = File::open(&path).map_err(|e| format!("Error opening database file: {}", e))?;
        let mut buffer = Vec::new();

        file.read(&mut buffer).map_err(|e| format!("Error reading database file: {}", e))?;

        if buffer[0..4] == MAGIC_NUMBER.to_le_bytes() {
            
            if buffer[4] >= 255 {
                return Err("Maximum number of tables reached (255)".to_string());
            }

            // header len + table num * one table len. Maybe this will remove in the future
            let offset: u8 = 10 + buffer[4] * 148;

            buffer[4] += 1; // Increment the number of tables
            
            let name_bytes_raw = _table_name.as_bytes();

            // Check if the table name is valid
            if name_bytes_raw.len() > 64 {
                return Err("Table name is too long, must be 64 bytes or less".to_string());
            }

            let mut name_bytes = Vec::with_capacity(64);
            name_bytes.extend_from_slice(name_bytes_raw);

            //we use null-padding right
            name_bytes.resize(64, 0u8);


            let path = format!("{}.tbl", _table_name);
            let mut file_path = Vec::with_capacity(68);
            file_path.extend_from_slice(path.as_bytes());

            return Ok(())
            
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
/// use objektDB::storage_engine::file_manager::delete_db;
///
/// match delete_db(String::from("my_database")) {
///     Ok(_) => println!("Database deleted successfully!"),
///     Err(e) => println!("Error deleting database: {}", e),
/// }
/// ```
pub fn delete_db(db_name: String) -> Result<(), String> {
    let db_name = db_name;
    let db_path = format!("{}/{}.db", db_name, db_name);
    
    if Path::new(&db_path).exists() {
        std::fs::remove_file(&db_path).map_err(|e| format!("Error deleting database: {}", e))?;
        Ok(())
    } else {
        Err(format!("Database {} does not exist", db_name))
    }
}

