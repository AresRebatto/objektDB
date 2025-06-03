trait CRUD: Sized{
    /// Selects all records from the database.
    fn select() -> Vec<Self>;

    ///It allows you to save changes made to objects to disk: 
    /// in particular, any object that undergoes a change is 
    /// identified as dirty.
    ///The moment 20 objects have been modified, the system 
    /// independently performs the update. If you want to run 
    /// it before the 20 objects are modified, you can use this method.
    ///In case the system aborts before the transaction has been saved 
    /// to disk, it relies on log files to succeed and recover the 
    /// situation. However, it is important to try not to be faced with 
    /// this situation, as it could result in errors: it would always be 
    /// preferable to rely on the update method.
    /// # Example
    /// ```
    /// use objektoDB::*;
    /// #[objekto("my_database.db")]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    /// let person = Person {
    ///     name: String::from("Alice"),
    ///     age: 30,
    /// };
    /// 
    /// match person.save(){
    ///     Ok(_) => println!("Person saved successfully!"),
    ///    Err(e) => println!("Error saving person: {}", e),
    /// }
    /// ```
    fn save() -> Result<(), String>;

    /// Filters records based on a condition.
    fn filter<F>(&self, condition: F) -> Vec<Self>
    where
        F: Fn(&Self) -> bool;

    /// Deletes a record from the database.
    fn delete(&self) -> Result<(), String>;
}