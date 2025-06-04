use crate::storage_engine::file_manager::create_db;


#[test]
fn test_create() {
    let create_db = create_db(String::from("test_db"));
    assert_eq!(create_db, Ok(()));
}