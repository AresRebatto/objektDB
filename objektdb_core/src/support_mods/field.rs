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

pub enum FieldType<T>{
    OID(i32),
    Primitive(T)
}


impl<T> FieldType<T>{
    pub fn unwrap_oid(&self)-> i32{
        match self{
            FieldType::OID(id)=> id.clone(),
            FieldType::Primitive(_)=>panic!("If you want to unwrap a primitive, you should use the unwrap_primitive() method.")
        }
    }

    pub fn unwrap_primitive(&self)-> T
    where
        T: Clone
    {
    match self{
        FieldType::OID(_)=> panic!("If you want to unwrap a oid, you should use the unwrap_oid() method."),
        FieldType::Primitive(val)=>val.clone()
    }
    }
}