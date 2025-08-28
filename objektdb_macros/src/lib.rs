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
    let name_lit_str = LitStr::new(&name.to_string(), Span::call_site());

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

    let fields_names: Vec<_> = if let Data::Struct(data) = &item.data{
        if let Fields::Named(named_field) = &data.fields{
            named_field.named.iter().map(|f|{
                let name = f.ident.as_ref().unwrap();
                quote!{
                    #name
                }
            })
        }else{
            panic!("Only named fields are supported");
        }
    }else{
        panic!("Only structs are supported");
    }.collect();


    let fields_types: Vec<syn::Type> = if let Data::Struct(data) = &item.data {
        if let Fields::Named(named_field) = &data.fields {
            named_field
                .named
                .iter()
                .map(|f| f.ty.clone()) 
                .collect()
        } else {
            panic!("Only named fields are supported");
        }
    } else {
        panic!("Only structs are supported");
    };

    let field_val = fields_types
        .iter()
        .zip(fields_names.iter())
        .map(|(t, n)| {
            quote! {
                if start >= data.len() { return None; }
                let dim = data[start] as usize;
                let next_start = start + 1;
                let end = next_start + dim;
                if end > data.len() { return None; }
                let #n: #t = <#t>::from_bytes(&data[next_start..end]);
                start = end;
            }
        });

    let expanded = quote! {
        impl objektdb::objektdb_core::traits::objekt::Objekt for #name{
            fn get_field_types() -> Vec<String>{
                vec![#(#field_type_literals.to_string()),*]
            }

            fn record_from_bytes(data: Vec<u8>)->Option<Self>{

                if data.is_empty(){
                    return None;
                }

                let mut start: usize = 0;
                
                #(
                    #field_val
                )*
               
               Some(Self{
                #(#fields_names: #fields_names),*
               })
                    
            }

            fn to_bytes(&self)-> Vec<u8>{
                todo!()
            }
        }


    };

    expanded.into()

}

#[proc_macro_attribute]
pub fn odb(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let db_name_lit: LitStr = parse_macro_input!(attr as LitStr);

    // fields will be Set<T> type. Here we'll put T
    let mut set_types: Vec<Type> = Vec::new();
    let mut set_types_literal: Vec<LitStr> = Vec::new();
    let mut params: Vec<proc_macro2::TokenStream> = Vec::new();


    //Extract T from Set<T>
    match &input.fields {
        syn::Fields::Named(field) => {
            for f in field.named.iter() {
                let f_name = &f.ident;
                let f_type = &f.ty;
                params.push(quote! { #f_name: #f_type });

                if let Type::Path(ty_path) = &f_type {
                    if let Some(segment) = ty_path.path.segments.last() {
                        if let PathArguments::AngleBracketed(ref generics) = segment.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = generics.args.first() {
                                let inner_ty_str = quote!(#inner_ty).to_string();
                                if !set_types.iter().any(|ty| quote!(#ty).to_string() == inner_ty_str) {
                                    set_types.push(inner_ty.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => panic!("The #[odb] macro can only be used with structures with named fields"),
    }

    set_types_literal = set_types
        .iter()
        .map(|t| {
            LitStr::new(t.to_token_stream().to_string().as_ref(), Span::call_site())
        })
        .collect();





    TokenStream::from(quote! {
        #input

        impl #struct_name {
            /// Create db and tables
            pub fn new() -> Result<(), String> {
                objektdb::objektdb_core::storage_engine::file_manager::create_db(#db_name_lit.to_string());

                let mut set_types: Vec<String> = Vec::new();


                Ok(())
            }
        }
    })
}



