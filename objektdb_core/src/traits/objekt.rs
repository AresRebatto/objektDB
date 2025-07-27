pub trait Objekt: Sized{

    fn get_field_types() -> Vec<String>;
    //fn select() -> Vec<Self>;

    //fn save() -> Result<(), String>;

    //fn filter<F>(&self, condition: F) -> Vec<Self>
    //where
    //    F: Fn(&Self) -> bool;

    //fn delete(&self) -> Result<(), String>;
}