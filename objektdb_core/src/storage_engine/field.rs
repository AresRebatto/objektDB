///The Field type represents, as the name suggests, 
///the struct fields to which the `objekt` macro is applied. 
///
///It is used by some functions within `file_manager`. 
///It grants some methods for handling fields, especially 
///with regard to reading from binary files.
pub struct Field{
    name: String,
    is_OID: bool,
    is_FK: bool,
    type_: String //come gestire le fk?
}