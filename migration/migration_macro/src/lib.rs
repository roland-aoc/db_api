use proc_macro::TokenStream;
use std::collections::HashMap;

use database::{self, ActiveConnection};

#[proc_macro_derive(Migrate)]
// #[proc_macro]
pub fn migrate_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_migrate_macro(&ast)
    // TokenStream::new()
}

fn impl_migrate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    println!("Struct name is {}", name);
    match &ast.data {
        syn::Data::Struct(d) => {
            let res = get_connection();
            match res {
                Ok(cnxn) => {
                    let schema: Result<HashMap<String, String>, MigrationError> = get_schema(cnxn);
                    let schema = match schema {
                        Ok(v) => v,
                        Err(e) => {println!("{:?}", e); return TokenStream::new()}
                    };
                    let structure = &d
                        .fields
                        .clone()
                        .into_iter()
                        .map(|f| {
                            let ident = f
                                .ident
                                .as_ref()
                                .expect("ident string couldn't be generated")
                                .to_string();
                            let ty = &f.ty;
                            let ty_str = quote::quote!(#ty).to_string();
                            (ident, ty_str)
                        })
                        .collect();
                    compare_states(&schema, &structure);
                }
                Err(e) => panic!("{:?}", e.kind),
            }
        }
        syn::Data::Enum(_) => panic!("not covered"),
        syn::Data::Union(_) => panic!("not covered"),
    }
    let generated = quote::quote! {
        impl Migrate for #name {
            fn migrate(&self) {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}

fn compare_states<'a>(
    schema: &'a HashMap<String, String>,
    structure: &'a HashMap<String, String>,
) -> HashMap<&'a str, &'a str> {
    let mut diff: HashMap<&str, &str> = HashMap::new();
    //check if structure's ks are already in schema
    for (k, v) in structure.iter() {
        if let None = schema.get(k) {
            diff.insert(&k, &v);
        }
    }
    println!("Current HashMap: {:?}", diff);
    //delete current fields
    // let mut diff: HashMap<String, String> = structure.iter()
    // .filter(|(&k, _)| schema.get(&k).is_none())
    // .collect();
    // }
    diff
}

fn get_schema(mut cnxn: database::Connection) -> Result<HashMap<String, String>, MigrationError> {
    let hash_map: HashMap<String, String> = cnxn
        .query_schema("account")
        .map_err(|e| MigrationErrorKind::Inner(e))
        .map_err(|kind| MigrationError { kind })?;
    Ok(hash_map)
    // {
    //     Ok(fields) => {
    //         Ok(fields)
    //         //TODO this was prbably in vain
    //         // for field in fields {
    //         //     hash_map.insert(
    //         //         field.0,
    //         //         database::util::parse_sql_field(&field.1)
    //         //             .map_err(|e| MigrationErrorKind::Inner(e))
    //         //             .map_err(|kind| MigrationError { kind })?,
    //         //     );
    //     },
    //     // Ok(hash_map)
    //     Err(e) => Err(MigrationError {
    //         kind: MigrationErrorKind::Inner(e),
    //     }),
}

// .map(|fields|
//     fields
//         .iter()
//         .map(|(k, v)| println!("{:?}, {:?}", k, v))
//         .map_error(|e| MigrationErrorKind::Inner(e).map_error(|kind| MigrationError(kind))))?;
// Ok(hash_map)
// }

#[derive(Debug)]
struct MigrationError {
    pub kind: MigrationErrorKind,
}

//TODO these can't be expoerte, what to do?
#[derive(Debug)]
#[non_exhaustive]
enum MigrationErrorKind {
    #[allow(dead_code)]
    Inner(database::DatabaseError),
}

fn get_connection() -> Result<database::Connection, MigrationError> {
    // :Result<Connection, Box<dyn std::error::Error>>

    match database::connect() {
        Ok(connection) => Ok(connection),
        Err(e) => Err(e)
            .map_err(|e| MigrationErrorKind::Inner(e))
            .map_err(|kind| MigrationError { kind }),
    }
}
