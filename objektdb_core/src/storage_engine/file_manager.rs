use std::fs::{File, self};
use std::io::{Read, Write};
use std::path::{Path};
use super::super::support_mods::{field::*};
use std::{env};


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

/// Creates a new database file and its directory structure.
///
/// This function initializes a new database by creating a directory named after `db_name`
/// and a corresponding `.db` file inside it. The database file is initialized with a binary
/// header containing metadata required for future operations.
///
/// The header format is as follows (total 10 bytes):
/// - Magic number (4 bytes, little-endian): Identifies the file as a valid objektDB database.
/// - Version (1 byte): Database format version.
/// - Number of tables (1 byte): Initially set to 0.
/// - Flags (4 bytes): Reserved for future use.
///
/// # Arguments
///
/// * `db_name` - The name of the database (without the `.db` extension). This will be used
///   both for the directory and the file name.
///
/// # Returns
///
/// * `Ok(())` if the database directory and file were successfully created and initialized.
/// * `Err(String)` if the directory or file could not be created, or if the database already exists.
///
/// # Errors
///
/// Returns an error if:
/// - The directory cannot be created (e.g., due to permissions or it already exists).
/// - The database file already exists.
/// - The file cannot be created or written to.
///
/// # Example
/// ```ignore
/// use objektDB::storage_engine::file_manager::create_db;
///
/// match create_db(String::from("my_database")) {
///     Ok(_) => println!("Database created successfully!"),
///     Err(e) => println!("Error creating database: {}", e),
/// }
/// ```
///
/// # Notes
/// - This function does not create any tables; use `create_table()` to add tables after database creation.
/// - The database directory will be created in the current working directory.
/// - If an error occurs after the directory is created but before the file is written, the directory may remain on disk.
pub fn create_db(db_name: String) -> Result<(), String> {
    //Work directory of developer
    let current_dir = env::current_dir()
        .map_err(|e| format!("Error getting current directory: {}", e))?;

    let db_dir = current_dir.join(&db_name);

    fs::create_dir(&db_dir)
        .map_err(|e| format!("Error creating database directory: {}", e))?;

    let db_file_path = db_dir.join(format!("{}.db", db_name));

    if db_file_path.exists() {
        return Err("The database already exists".to_string());
    }

    match File::create(&db_file_path) {
        Err(e) => {
            let _ = fs::remove_dir(&db_dir); // pulizia
            Err(format!("Error creating database file: {}", e))
        },
        Ok(mut file) => {
            let mut buffer = Vec::with_capacity(10);

            buffer.extend_from_slice(&MAGIC_NUMBER.to_le_bytes()); // Magic number
            buffer.extend_from_slice(&[1u8]); // Version
            buffer.extend_from_slice(&[0u8]); // Number of tables
            buffer.extend_from_slice(&[0u8; 4]); // Flags

            file.write_all(&buffer)
                .map_err(|e| format!("Failed to write database header: {}", e))
        }
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
/// HEADER{
///	    struct_name,
///	    OffsetHeader,
///	    OffsetIndex,
///	    OffsetBucket,
///     last_OID
///	    References{
///         references_n
///	    	ClassName1,(max 64 bytes)
///	    	ClassName2
///	    }
///	    StructStructure{
///	    	(In the data section will be added OID on the top for every istance
///         as a u32)
///         length(for switch directly to method
///             names without reading all fields
///             and their length)
///         {
///             length (2 byte)
///             field1
///             is_fk
///             type
///         }
///	        {
///             length
///             field2
///             is_fk
///             type         
///         }
///         {
///             length (8 byte)
///             methodname1
///         }
///         {
///             length
///             methodname2
///         }
///	    }
///}
/// ```
/// # Arguments
/// * `_table_name` - The name of the table to be created.
/// * `_db_name` - The name of the database where the table will be created (without the `.db` extension).
/// # Returns
/// * `Ok(())` if the table was successfully created.
/// * `Err(String)` if the database file does not exist, is invalid, or if an error occurred during the process.
/// # Example
/// ```ignore
/// use objektDB::storage_engine::file_manager::create_table;
/// match create_table(String::from("my_table"), String::from("my_database")) {
///    Ok(_) => println!("Table created successfully!"),
///    Err(e) => println!("Error creating table: {}", e),
/// }
/// ```
/// 
pub fn create_table(_table_name: String, _db_name: String, _ref: Vec<String>, _fields: Vec<Field>, _methods_names: Vec<String>) -> Result<(), String> {
    
    let current_dir = env::current_dir()
        .map_err(|e| format!("Error getting current directory: {}", e))?;

    //CHANGES TO DB FILE
    let path = current_dir.join(format!("{}/{}.db", _db_name, _db_name));

    // Check if the database file exists
    if Path::new(&path).exists() {

        let mut file: File = File::open(&path).map_err(|e| format!("Error opening database file: {}", e))?;
        let mut buffer: Vec<u8> = Vec::new();

        file.read(&mut buffer).map_err(|e| format!("Error reading database file: {}", e))?;

        if buffer[0..4] == MAGIC_NUMBER.to_le_bytes() {
            
            if buffer[4] >= 255 {
                return Err("Maximum number of tables reached (255)".to_string());
            }



            buffer[4] += 1; // Increment the number of tables

            let _ = file.write(&buffer);


            //CHANGES TO TBL FILE
            
            //convert to UTF-8
            let name_bytes_raw: &[u8] = _table_name.as_bytes();

            if name_bytes_raw.len() > 64 {
                return Err("Table name is too long, must be 64 bytes or less".to_string());
            }
            
            //we use null-padding left
            let mut name_bytes: Vec<u8> = vec![0u8; 64-_table_name.len() as usize];
            name_bytes.extend_from_slice(name_bytes_raw);


            let path = format!("{}/{}.tbl",_db_name, _table_name);
            
            //Lenth: 1(8 bit for the num of fields) + num_fields*64
            let mut references_field: Vec<u8> = Vec::new();

            //First byte is num of references
            references_field.push(_ref.len() as u8);

            for r in &_ref {
                
                //Null padding left
                let mut ref_bytes = vec![0u8; 64-r.as_bytes().len() as usize];
                ref_bytes.extend_from_slice(r.as_bytes());
                references_field.extend_from_slice(&ref_bytes);

            }

            
            
            //length_field+field+is_fk+length_type+type
            let mut fields: Vec<u8> = Vec::new();

            //length_fields
            let mut tot_len: Vec<u8> = Vec::new();

            for field in _fields{

                //name
                fields.push(field.name.len() as u8);
                fields.extend_from_slice(field.name.as_bytes());

                fields.push(field.is_FK as u8);

                //type
                fields.push(field.type_.len() as u8);
                fields.extend_from_slice(field.type_.as_bytes());
            }
            
            tot_len.extend_from_slice(&(fields.len() as u16).to_le_bytes());
            

            let mut methods: Vec<u8> = Vec::new();

            for method in _methods_names{
                methods.push(method.len() as u8);
                methods.extend_from_slice(method.as_bytes());
            }

            let offset_header = ((76+references_field.len()+fields.len()+methods.len()) as u32).to_le_bytes();
            
            let mut header: Vec<u8> = Vec::new();


            header.extend_from_slice(&name_bytes);
            header.extend_from_slice(&offset_header);
            header.extend_from_slice(&[0u8; 3]);
            header.extend_from_slice(&references_field);
            header.extend_from_slice(&fields);
            header.extend_from_slice(&methods);

            //header+index
            let tbl_file = [header, vec![0u8; 262144 as usize]].concat();
            match File::create(path){
                Err(e)=> Err(format!("The table could not be created: {}", e)),
                Ok(mut f)=>{

                    if let Err(e)= f.write(&tbl_file){
                        return Err(format!("Error creating the .tbl file: {}", e));
                    }
                    return Ok(());
                }
            }

            
            
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
/// ```ignore
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


///Allows you to reinitialize a table.
///
///It must be called **manually** by the developer 
///when applying some change to the structure of one 
///of the structs. The method clears the table and reinitializes it.
///
///**Caution**: the method deletes all data within the table.
pub fn reinitialize_table(_table_name: String, _db_name: String, _ref: Vec<String>, _fields: Vec<Field>)-> Result<(), String>{
    todo!()
}