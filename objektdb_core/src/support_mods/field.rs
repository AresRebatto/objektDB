///The Field type represents, as the name suggests, 
///the struct fields to which the `objekt` macro is applied. 
///
///It is used by some functions within `file_manager`. 
///It grants some methods for handling fields, especially 
///with regard to reading from binary files.
pub struct Field{
    pub name: String,
    pub is_OID: bool,
    
}



pub struct OID{
    val: i32
}

pub struct Primitive<T>{
    val: T
}