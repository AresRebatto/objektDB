mod traits;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct, LitStr};

#[proc_macro_attribute]
pub fn objekt_impl(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    
    _item
}

///It should be inserted on top of your structs to declare that it is an entity whose 
/// instances you want to store. The database in which you want to store the data should 
/// be specified.
///If the specified database does not exist, it is automatically created: so if you mistype 
/// the database name and enter one that does not exist, a new one will automatically be 
/// created. For the moment , since a language for interacting with data directly is not yet 
/// available, to delete a database you can make use of the function `delete_db(“database_name”)`.
/// 
/// In addition, it also implements CRUD trait to perform transactions on the database.
/// In particular, it implements the following functions:
/// - `select()`
/// - `update()`
/// - `where()`
/// - `delete()`
/// # Example
/// ```
/// use objektoDB::*;
/// #[objekt("my_database.db")]
/// struct Person {
///    name: String,
///    age: u32,
/// }
#[proc_macro_attribute]
pub fn objekt(_attr: TokenStream, _item    : TokenStream) -> TokenStream {
    let input = parse_macro_input!(_item as ItemStruct);
    let struct_name = &input.ident;

     let db_name_lit = parse_macro_input!(_attr as LitStr);
    let db_name = db_name_lit.value();


    //Qui sarà necessario chiamare la funzione che crea il database


    let expanded = quote::quote! {
        #input

        impl traits::CRUD for #struct_name {
            //da rivedere
            fn select() -> Vec<Self> {
                helper::select::<#struct_name>(&#db_name)
            }

            fn save(&self) -> Result<(), String> {
                helper::save(self, &#db_name)
            }

            fn filter<F>(&self, condition: F) -> Vec<Self>
            where
                F: Fn(&Self) -> bool,
            {
                helper::filter(self, condition)
            }

            fn delete(&self) -> Result<(), String> {
                helper::delete(self, &#db_name)
            }
        }

    };

    TokenStream::from(expanded)
}

