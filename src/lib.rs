///It provides the functions and all those components that 
/// are needed at runtime and that could not for that reason 
/// be included in a crate that has enabled the use of procedural macros.
pub use objektdb_core;

///It provides all the useful macros for extracting data 
/// from structures and their implementations, as well as 
/// for implementing CRUD operations efficiently.
pub use objektdb_macros;