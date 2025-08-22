pub trait Objekt: Sized{

    fn get_field_types() -> Vec<String>;
    fn from_bytes(data: Vec<u8>)-> Option<Self>;
    fn to_bytes(&self)-> Vec<u8>;


}