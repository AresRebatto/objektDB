use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input,
    parse_str,
    Data,
    DeriveInput,
    Fields,
    GenericArgument,
    Ident,
    ItemStruct,
    LitStr,
    PathArguments,
    Type
};
use proc_macro2;
use proc_macro2::Span;

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
/// ```ignore
/// use objektDB::*;
/// #[objekt("my_database.db")]
/// struct Person {
///    name: String,
///    age: u32,
/// }
#[proc_macro_derive(Objekt)] //Need to change to derive macro(Change in architecture)
pub fn objekt_derive(input: TokenStream) -> TokenStream {

    let item = parse_macro_input!(input as DeriveInput);
    let name = &item.ident;

    let field_type_literals: Vec<LitStr> = if let Data::Struct(data) = &item.data {
        if let Fields::Named(named) = &data.fields {
            named.named.iter().map(|f| {
                let ty = &f.ty;
                let base = if let syn::Type::Path(type_path) = ty {
                    type_path.path.segments.first().unwrap().ident.to_string()
                } else {
                    quote!{#ty}.to_string()
                };
                LitStr::new(&base, Span::call_site())
            }).collect()
        } else {
            panic!("Only named fields are supported");
        }
    } else {
        panic!("Only structs are supported");
    };
    let expanded = quote! {
        impl objektdb::objektdb_core::traits::objekt::Objekt for #name{
            fn get_field_types() -> Vec<String>{
                vec![#(#field_type_literals.to_string()),*]
            }
        }
    };

    expanded.into()

}

#[proc_macro_attribute]
pub fn odb(attr: TokenStream, item: TokenStream) -> TokenStream{

    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let mut fields: Vec<(String, String)> = Vec::new();

    //fields will be Set<T> type. Here we'll put T
    let mut sets_type: Vec<Type> = Vec::new();

    match &input.fields {
        syn::Fields::Named(field) => {
            for f in field.named.iter() {
                let f_name = &f.ident;
                let f_type = &f.ty;
                fields.push((
                    f_name.as_ref()
                        .unwrap()
                        .to_string(),
                    f_type
                        .to_token_stream()
                        .to_string()
                ));

                //Verify that the field type is set and extract the generic type
                if let Type::Path(ty_path) = &f_type{

                    if let Some(seg) = ty_path.path.segments.last(){

                        if seg.ident == "Set"{

                            if let PathArguments::AngleBracketed(args) = &seg.arguments{

                                if let Some(GenericArgument::Type(inner_ty)) = args.args.first(){
                                    sets_type.push(inner_ty.clone());
                                }
                            }

                        }else{
                            panic!("Fields in a struct that use #[odb] must always be of type Set<T>")
                        }
                    }
                }


            }
        },
        _ => panic!("The #[objekt] macro can only be used with structures with named fields"),
    }

    let lit_types: Vec<LitStr> = fields
        .iter()
        .map(|(_, v)| LitStr::new(v, proc_macro2::Span::call_site()))
        .collect();

    let types = fields.iter().map(|(name_str, ty_str)| {
        let ident = Ident::new(name_str, proc_macro2::Span::call_site());
        let ty: Type = parse_str(ty_str).expect("Failed to parse type");
        quote! { #ty }
    });
    let db_name_lit: LitStr = parse_macro_input!(attr as LitStr);



    let params = fields.iter().map(|(name_str, ty_str)| {
        let ident = Ident::new(name_str, proc_macro2::Span::call_site());
        let ty: Type = parse_str(ty_str).expect("Failed to parse type");
        quote! { #ident: #ty }
    });
    TokenStream::from(quote::quote!{
        #input

        impl #struct_name {

            ///Create db
            pub fn new()-> Result<(), String>{
                objektdb::objektdb_core::storage_engine::file_manager::create_db(#db_name_lit.to_string());

                Ok(())
            }
        }

    })


}


