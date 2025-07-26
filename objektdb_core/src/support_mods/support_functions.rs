pub fn converter_builder(types: Vec<String>)-> Result<(), String>{
    Ok(())
}

///Returns true if the type passed as a parameter is
/// a reference (and is therefore not one of Rust's standard types) or false otherwise.
///
///This function is useful for detecting reference types, i.e., those that refer to other
/// custom structures.
///
/// Any type not included in the following will be considered as a reference.
/// - `i8`
/// - `i16`
/// - `i32`
/// - `i64`
/// - `i128`
/// - `isize`
/// - `u8`
/// - `u16`
/// - `u32`
/// - `u64`
/// - `u128`
/// - `usize`
/// - `f32`
/// - `f64`
/// - `char`
/// - `String`
/// - `&str`
/// - `bool`
pub fn find_reference(type_: String)-> bool{
    return match type_.as_str(){
        "i8" => false,
        "i16" => false,
        "i32" => false,
        "i64" => false,
        "i128" => false,
        "isize" => false,
        "u8" => false,
        "u16" => false,
        "u32" => false,
        "u64" => false,
        "u128" => false,
        "usize" => false,
        "f32" => false,
        "f64" => false,
        "char" => false,
        "String" => false,
        "&str" => false,
        "bool" => false,
        _ => true
    };

    todo!()
}