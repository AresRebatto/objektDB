pub trait Objekt: Sized{

    fn get_field_types() -> Vec<String>;
    fn record_from_bytes(data: Vec<u8>)-> Option<Self>;
    fn to_bytes(&self)-> Vec<u8>;
    
    //for creating the table(using file_manager::crate_table())
    fn new(struct_name: String)-> Result<(), String>;


}