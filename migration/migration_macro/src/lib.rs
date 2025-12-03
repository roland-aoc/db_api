use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;

use database::{self, ActiveConnection, DdlType};
use std::fs;
use std::path::PathBuf;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(Migrate)]
pub fn migrate_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_migrate_macro(&ast)
}

fn impl_migrate_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let struct_name = &ast.ident;
    let guard_name = quote::format_ident!(
        "__{}_MIGRATION_GUARD",
        struct_name.to_string().to_uppercase()
    );

    let fields_build = match &ast.data {
        Data::Struct(d) => {
            let marker_file = get_marker_path(&name.to_string());

            let _changes_as_rust_types = if !marker_file.exists() {
                let mut cnxn = get_connection().expect("Connection could't be gotten!");
                let schema: HashMap<String, String> =
                    get_schema(&mut cnxn).expect("Couldn't get schema!");

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

                let structure_map: HashMap<String, String> = structure.iter().cloned().collect();

                let changes = compare_states(&schema, &structure_map);

                if !changes.is_empty() {
                    cnxn.execute_ddl(database::Ddl::new(
                        DdlType::Create,
                        &name.to_string(),
                        &changes,
                    ));
                }
                fs::create_dir_all(marker_file.parent().unwrap()).ok();
                fs::write(&marker_file, "migrated").ok();

                changes
            } else {
                HashMap::new()
            };

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

            quote! { #(#fields_as_tokens)* }
        }
        _ => quote! {},
    };

    let generated = quote! {
        static #guard_name: std::sync::Once = std::sync::Once::new();

        impl migration::Migrate for #struct_name {
            fn migrate(&self) {
                // Noop
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

fn get_marker_path(struct_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push(".migration_markers");
    path.push(format!("{}.marker", struct_name.to_lowercase()));
    path
}

fn compare_states(
    schema: &HashMap<String, String>,
    structure: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut changes: HashMap<String, String> = HashMap::new();
    //check if structure's ks are already in schema
    for (k, v) in structure.iter() {
        if !schema.contains_key(k) {
            changes.insert(k.clone(), v.clone());
        }
    }
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
