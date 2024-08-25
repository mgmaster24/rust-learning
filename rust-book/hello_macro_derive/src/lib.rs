use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(HelloMacro)]
pub fn hell_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // In production we should like panic or use expect instead
    // of unwrap
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
      impl HelloMacro for #name {
        fn hello_macro() {
           println!("Hello, Macro!  My name is {}!", stringify!(#name));
        }
      }
    };

    gen.into()
}
