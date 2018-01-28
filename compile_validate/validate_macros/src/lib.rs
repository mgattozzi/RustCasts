extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde;
extern crate serde_json;
extern crate syn;

use std::str::FromStr;
use std::fs::File;
use std::io::Read;

use proc_macro::TokenStream;
use proc_macro2::TokenNode;
use quote::ToTokens;
use serde_json::Value;
use syn::DeriveInput;

#[proc_macro_derive(Validate, attributes(ValidatePath))]
pub fn validate_file(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let mut ast: DeriveInput = syn::parse(input)
                                   .expect("validate_macros: Failed to parse Derive Input");

    let attr = ast.attrs.pop().unwrap();
    attr.tts
        .into_iter()
        .skip(1)
        .for_each(|tt| {
            if let TokenNode::Literal(lit) = tt.kind {
                let mut path = lit.to_string();
                let _ = path.pop();
                path.remove(0);
                let mut file = File::open(&path).unwrap();
                let mut buffer = String::new();
                let _ = file.read_to_string(&mut buffer);

                let _: Value = serde_json::from_str(&buffer).unwrap();
            } else {
                panic!("validate_macros: Path not passed int to Validate Macro");
            }
        });

    TokenStream::from_str(&"".into_tokens().to_string())
                .expect("validate_macros: Failed to generate blank TokenStream")
}
