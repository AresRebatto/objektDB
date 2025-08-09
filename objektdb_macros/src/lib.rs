use proc_macro::TokenStream;
use quote::{quote, ToTokens, format_ident};
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
    let db_name_lit: LitStr = parse_macro_input!(attr as LitStr);


    //fields will be Set<T> type. Here we'll put T
    let mut set_types: Vec<Type> = Vec::new();
    let mut set_types_literal: Vec<LitStr> = Vec::new();

    let mut params: Vec<proc_macro2::TokenStream> = Vec::new();

    match &input.fields {
        syn::Fields::Named(field) => {
            for f in field.named.iter() {

                let f_name = &f.ident;
                let f_type = &f.ty;
                params.push(quote!{#f_name: #f_type});


                if let Type::Path(ty_path) = &f_type{
                    if let Some(segment) = ty_path.path.segments.last() {
                        if let PathArguments::AngleBracketed(ref generics) = segment.arguments {

                            if let Some(GenericArgument::Type(inner_ty)) = generics.args.first() {

                                let mut already_exists = false;
                                let inner_ty_str = quote!(#inner_ty).to_string();

                                //Check that the value is not already present in set _types.
                                for ty in &set_types {
                                    if quote!(#ty).to_string() == inner_ty_str {
                                        already_exists = true;
                                        break;
                                    }
                                }

                                if !already_exists{
                                    set_types.push(inner_ty.clone());
                                }
                            }
                        }
                    }
                }
            }
        },
        _ => panic!("The #[objekt] macro can only be used with structures with named fields"),
    }

    set_types_literal = set_types.iter().map(|t|{
        LitStr::new(
            t.to_token_stream()
                .to_string()
                .as_ref(), proc_macro2::
            Span::call_site())
    }).collect();


    let blocks = set_types.iter().zip(set_types_literal.iter()).map(|(t, lit)| {
        quote! {
            for field in #t::get_field_types() {
                if field == #lit {
                    references.push(field.clone());
                }
            }
        }
    });


    let convert_trait = format_ident!("{}Convert", struct_name);

    let convert_trait_impl: Vec<_> = set_types.iter().map(|t|{
        quote!{
            impl #convert_trait for #t{
                fn convert_reference(val: String, ty: String)-> Box<KnownTypes>{
                    todo!();
                }
            }
        }
    }).collect();

    let known_types: Vec<_> = set_types.iter().map(|t|{
        quote! {
            #t(#t)
        }
    }).collect();
    TokenStream::from(quote::quote!{
        #input

        impl #struct_name {

            ///Create db and tables
            pub fn new()-> Result<(), String>{
                objektdb::objektdb_core::storage_engine::file_manager::create_db(#db_name_lit.to_string());

                let mut references: Vec<String> = Vec::new();
                let mut set_types: Vec<String> = Vec::new();
                //let mut fields: Vec<String> = Vec::new();

                #(#blocks)*
                //#(fields.push(#set_types_literal.to_string());)*

                Ok(())
            }

        }

        pub trait #convert_trait{
            fn convert_reference(val: String, ty: String)->Box<KnownTypes>{
                todo!()
            };
        }

        #(#convert_trait_impl)*

        pub enum KnownTypes{
            #(#known_types),*
        }

    })


}




