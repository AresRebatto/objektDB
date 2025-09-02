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
    Type,
    ItemImpl,
    ImplItem,
    TypePath
};
use proc_macro2::{self};
use proc_macro2::Span;

#[proc_macro_attribute]
pub fn objekt_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty(){
        panic!("This macro should not have any attributes.");
    }
    let impl_block = parse_macro_input!(input as ItemImpl);

    let methods_names: Vec<String> = impl_block.items.iter().filter(|item| matches!(item, ImplItem::Fn(_)))
                                .map(|item| {
                                    if let ImplItem::Fn(method) = item {
                                        method.sig.ident.clone().to_string()
                                    } else {
                                        unreachable!() // Non dovrebbe mai succedere per via del filter
                                    }
                                })
                                .collect();

    let struct_name = &impl_block.self_ty;

    let expanded = quote!{

        impl objektdb::objektdb_core::traits::impl_block for #struct_name{
            pub fn get_methods_names()-> Vec<String>{
                let mut res: Vec<String> = Vec::new();

                #(res.push(#methods_names.to_string());)*

                return res;
            }
        }
    };

    expanded.into()


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

    let fields_inner_types: Vec<_> = fields_types.iter().map(|ty| {
        match ty {
            Type::Path(type_path) => {
                if let Some(last_segment) = type_path.path.segments.last() {
                    let type_name = &last_segment.ident;
                    
                    match type_name.to_string().as_str() {
                        "OID" => {
                            if !matches!(last_segment.arguments, syn::PathArguments::None) {
                                panic!("OID must not have generic parameters");
                            }
                            syn::parse_quote!(i32)
                        }
                        "Primitive" => {
                            if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                                    inner_type.clone()
                                } else {
                                    panic!("Primitive must have exactly one generic type parameter T");
                                }
                            } else {
                                panic!("Primitive must be parameterized with a generic type");
                            }
                        }
                        _ => {
                            panic!("Unsupported type: '{}'. Only OID and Primitive<T> are supported", type_name);
                        }
                    }
                } else {
                    panic!("Unable to determine type name from path");
                }
            }
            _ => {
                panic!("Unsupported type. Only path types (OID and Primitive<T>) are supported");
            }
        }
    }).collect();

    let fields_names_literals: Vec<_> = fields_inner_types.iter().map(|n|{
        LitStr::new(n.to_token_stream().to_string().as_ref(), Span::call_site())
    }).collect();

    let field_val = fields_types
    .iter()
    .zip(fields_names.iter())
    .zip(fields_inner_types)
    .map(|((t, n), inner_ty)| {
        // Determina se il tipo ha generics
        let constructor = match t {
            Type::Path(type_path) => {
                if let Some(last_segment) = type_path.path.segments.last() {
                    match last_segment.ident.to_string().as_str() {
                        "OID" => {
                            quote! {
                                objektdb::objektdb_core::support_mods::field::OID {
                                    val: #inner_ty::from_bytes(&data[next_start..end])
                                }
                            }
                        }
                        "Primitive" => {
                            quote! {
                                objektdb::objektdb_core::support_mods::field::Primitive::<#inner_ty> {
                                    val: #inner_ty::from_bytes(&data[next_start..end])
                                }
                            }
                        }
                        _ => panic!("Unsupported type")
                    }
                } else {
                    panic!("Unable to determine type")
                }
            }
            _ => panic!("Unsupported type")
        };

        quote! {
            if start >= data.len() { return None; }
            let dim = data[start] as usize;
            let next_start = start + 1;
            let end = next_start + dim;
            if end > data.len() { return None; }
            let #n = #constructor;
            start = end;
        }
    });

    let mut methods_n;
    #[cfg(feature="impl_blocks")]{
        methods_n = quote! {
            let methods_names = Self::get_methods_names();
        };
    }

    #[cfg(not(feature="impl_blocks"))]{
        methods_n = quote! {
            let methods_names = vec![];
        };
    }

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

            fn new(struct_name: String)-> Result<(), String>{
                
               #methods_n


                objektdb::objektdb_core::storage_engine::file_manager::create_table(
                    #name_lit_str.to_string(), 
                    struct_name, 
                    vec![#(#fields_names_literals.to_string()),*], 
                    methods_names

                )
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



