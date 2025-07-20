mod traits;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemStruct, LitStr};
use objektdb_core::storage_engine::file_manager;

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
/// use objektDB::*;
/// #[objekt("my_database.db")]
/// struct Person {
///    name: String,
///    age: u32,
/// }
#[proc_macro_attribute]
pub fn objekt(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let mut fields: Vec<(syn::Ident, syn::Type)> = Vec::new();
    
    match &input.fields {
        syn::Fields::Named(field) => {
            for f in field.named.iter() {
                let f_name = f.ident.clone().unwrap();
                let f_type = f.ty.clone();
                fields.push((f_name, f_type));
            }
        },
        _ => panic!("The #[objekt] macro can only be used with structures with named fields"),
    }

    let db_name_lit = parse_macro_input!(attr as LitStr);
    let db_name = db_name_lit.value();
    

    let params = fields.iter().map(|(name, ty)| quote! { #name: #ty });

    let expanded = quote::quote! {
        
        #input

        impl #struct_name {

            pub fn new( #( #params ),* ) -> Result<(), String> {
                let _ = file_manager::create_db(#db_name.to_string());
                //file_manager::create_table();

                Ok(())
                
            }
        }
    };

    TokenStream::from(expanded)
}

