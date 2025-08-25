pub(crate) fn string_padding(vec: &mut Vec<u8>, str: String, tot_len: usize)-> Result<(), String>{
    if tot_len < str.len(){
        return Err(String::from("string too long"));
    }

    vec.extend_from_slice(&vec![0u8; tot_len-str.len()]);
    vec.extend_from_slice(str.as_bytes());
    Ok(())
}


