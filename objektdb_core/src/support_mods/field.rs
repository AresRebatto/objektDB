///The Field type represents, as the name suggests, 
///the struct fields to which the `objekt` macro is applied. 
///
///It is used by some functions within `file_manager`. 
///It grants some methods for handling fields, especially 
///with regard to reading from binary files.
pub struct Field{
    pub name: String,
    pub is_OID: bool,
    pub is_FK: bool,
    pub type_: String //FK types will be managed by a function that will create a kind of register in a
    //different rust file where will be create a match pattern for convert fk values in effective values
}

pub enum FieldType{
    OID,
    ForeignKey{reference: String},
    Primitive(String)
}