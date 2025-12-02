use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;

use database::{self, ActiveConnection, DdlType};
use syn::{self, Data, DeriveInput};

#[proc_macro_derive(Migrate)]
pub fn migrate_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_migrate_macro(&ast)
}

fn impl_migrate_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Build code to push database::domain::Field entries at runtime.
    let fields_build = match &ast.data {
        Data::Struct(d) => {
            let mut cnxn = get_connection().expect("Connection could't be gotten!"); //TODO add errior handling
            let schema: HashMap<String, String> =
                get_schema(&mut cnxn).expect("Couldn't get schema!"); //TODO add error handling

            let structure: Vec<(String, String)> = d
                .fields
                .iter()
                .filter_map(|f| {
                    let _ident = f.ident.as_ref()?;
                    let ty = &f.ty;
                    Some((
                        stringify!(#_ident).to_string(),
                        quote::quote!(#ty).to_string(),
                    ))
                })
                .collect();

            let structure_map: HashMap<String, String> = structure
            .iter()
            .cloned()
            .collect();

            let changes_as_rust_types = compare_states(&schema, &structure_map);

            let fields_as_tokens = structure
                .iter()
                .map(|(name, ty)| {
                    quote! {
                        fields.push(database::domain::Field::new(
                            #name.to_string(),
                            #ty.to_string()
                        ));
                    }
                })
                .collect::<Vec<_>>();

            cnxn.execute_ddl(database::Ddl::new(
                DdlType::Create,
                &name.to_string(),
                &changes_as_rust_types,
            ));
            quote! { #(#fields_as_tokens)* }
        }
        _ => quote! {},
    };

    let generated = quote! {
        impl migration::Migrate for #name {
            fn migrate(&self) {
                // no-op
            }

            fn get_fields(&self) -> Vec<database::domain::Field> {
                let mut fields = Vec::new();
                #fields_build
                fields
            }
        }
    };
    generated.into()
}

fn compare_states<'a>(
    schema: &'a HashMap<String, String>,
    structure: &'a HashMap<String, String>,
) -> HashMap<&'a String, &'a String> {
    let mut changes: HashMap<&'a String, &'a String> = HashMap::new();
    //check if structure's ks are already in schema
    for (k, v) in structure.iter() {
        if let None = schema.get(&k.to_string()) {
            changes.insert(k, v);
        }
    }
    // println!("Current HashMap: {:?}", changes);
    //delete current fields

    // let mut diff: HashMap<String, String> = structure.iter()
    // .filter(|(&k, _)| schema.get(&k).is_none())
    // .collect();
    // }
    changes
}

fn get_schema(
    cnxn: &mut database::Connection,
) -> Result<HashMap<String, String>, InternalMigrationError> {
    let hash_map: HashMap<String, String> = cnxn
        .query_schema("account")
        .map_err(|e| InternalMigrationErrorKind::Inner(e))
        .map_err(|kind| InternalMigrationError { kind })?;
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
#[allow(dead_code)]
struct InternalMigrationError {
    pub kind: InternalMigrationErrorKind,
}

//TODO these can't be expoerte, what to do?
#[derive(Debug)]
#[non_exhaustive]
enum InternalMigrationErrorKind {
    #[allow(dead_code)]
    Inner(database::DatabaseError),
}

fn get_connection() -> Result<database::Connection, InternalMigrationError> {
    // :Result<Connection, Box<dyn std::error::Error>>

    match database::connect() {
        Ok(connection) => Ok(connection),
        Err(e) => Err(e)
            .map_err(|e| InternalMigrationErrorKind::Inner(e))
            .map_err(|kind| InternalMigrationError { kind }),
    }
}
