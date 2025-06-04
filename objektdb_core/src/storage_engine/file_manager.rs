use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

const MAGIC_NUMBER: u32 = 0x4D594442;

///Creates a new database from the name. It initially creates it empty, and 
/// you can then insert tables using the `create_table()` method.
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
///If the conversion is successful, it returns `Ok(())`, otherwise `Err(error)`
pub fn create_db(_name : String) -> Result<(), String> {
    let db_path = format!("{}.db", _name);
    if !Path::new(&db_path).exists() {

        match OpenOptions::new().write(true).create_new(true).open(&db_path) {
            Ok(f) => {
                let mut buffer = Vec::new();

                //header
                buffer.extend_from_slice(&MAGIC_NUMBER.to_le_bytes());
                buffer.extend_from_slice(&[1 as u8]); //Version
                buffer.extend_from_slice(&[0u8; 1]); //Number of tables
                buffer.extend_from_slice(&[0u8; 4]); //flags(for future use)

                
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

