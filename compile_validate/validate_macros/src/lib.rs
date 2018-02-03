#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate serde;
extern crate serde_json;
extern crate syn;

use std::fs::File;

use proc_macro::TokenStream;
use serde::de::IgnoredAny;
use syn::{DeriveInput, Lit, Meta, MetaNameValue};

#[proc_macro_derive(Validate, attributes(path))]
pub fn validate_file(input: TokenStream) -> TokenStream {
    // Parse tokens of the input struct
    let ast: DeriveInput = syn::parse(input)
                               .expect("validate_macros: Failed to parse Derive Input");

    // Interpret unstructured tokens of the `#[path = $path]` attribute into a
    // structured meta-item representation
    let meta = ast.attrs[0].interpret_meta().unwrap();

    // Expect the `$path` to be a string literal
    let path = match meta {
        Meta::NameValue(MetaNameValue { lit: Lit::Str(s), .. }) => s.value(),
        _ => panic!("validate_macros: Expected #[path = \"...\"]"),
    };

    // Validate JSON without parsing it into any particular data structure
    let file = File::open(&path).unwrap_or_else(|err| panic!("{}", err));
    match serde_json::from_reader(file) {
        Ok(IgnoredAny) => {}
        Err(err) => panic!("Failed to parse {}: {}", path, err),
    }

    // If successful, produce empty token stream as output
    quote!().into()
}
