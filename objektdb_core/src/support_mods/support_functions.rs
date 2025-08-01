pub fn converter_builder(types: Vec<String>)-> Result<(), String>{
    Ok(())
}


pub(crate) fn padding(vec: &mut Vec<u8>, str: String, tot_len: usize)-> Result<(), String>{
    if tot_len < str.len(){
        return Err(String::from("string too long"));
    }

    vec.extend_from_slice(&vec![0u8; tot_len-str.len()]);
    vec.extend_from_slice(str.as_bytes());
    Ok(())
}