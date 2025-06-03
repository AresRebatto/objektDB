use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn create_db(_name : String) -> Result<(), String> {
    let db_path = format!("{}.db", _name);
    if !Path::new(&db_path).exists() {

        match OpenOptions::new().write(true).create_new(true).open(&db_path) {
            Ok(f) => Ok(()),
            Err(e) => return Err(format!("Error in the file creation: {}", e)),
        }


    } else {
        return Err(format!("Database {} already exists", _name));
    }   

    
}   

pub fn create_table(_table_name: String, _db_name: String) -> Result<(), String> {
    todo!();
}

