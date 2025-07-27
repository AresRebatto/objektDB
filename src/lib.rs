//For macros crate
pub use objektdb_core;
pub use objektdb_macros;

pub use objektdb_macros::{Objekt, objekt_impl, odb};
pub use objektdb_core::{
    storage_engine::{
        file_manager::{
            create_db, 
            create_table, 
            delete_db, 
            reinitialize_table
        }
    }, 
    support_mods::{
        support_functions::*,
        field::*,
        set::*
    }
};