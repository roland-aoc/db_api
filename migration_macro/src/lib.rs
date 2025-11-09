use proc_macro::TokenStream;

#[proc_macro_derive(Migrate)]
// #[proc_macro]
pub fn migrate_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    // match &ast {
    //     Ok(v) => impl_hello_macro(v),
    //     Err(_) => panic!("dont understand")
    // };
    // let ast = syn::parse_macro_input!(input as Item);
    impl_migrate_macro(&ast)
    // TokenStream::new()
}

fn impl_migrate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // let generated = quote::quote! {
    println!("Hello, Macro! My name is {}!", name);
     match &ast.data {
        syn::Data::Struct(d) => for field in &d.fields {
            println!("{:?}", field)
        },
        syn::Data::Enum(_) => panic!("not covered"),
        syn::Data::Union(_) => panic!("not covered")
        
     }
    let generated = quote::quote! {
        impl Migrate for #name {
            fn migrate(&self) {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // println!("stringify {}!", ast);
    // }
    generated.into()
    // TokenStream::new()
}