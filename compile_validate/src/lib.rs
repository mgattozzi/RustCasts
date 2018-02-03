// Re-export the procedural macro so that users don't need to have both this
// crate and validate_macros in their Cargo.toml.
#[allow(unused_imports)]
#[macro_use]
extern crate validate_macros;
pub use validate_macros::*;

#[macro_export]
macro_rules! validate_json {
    ($path: expr) => {
        #[allow(dead_code)]
        mod dsjfkdsjfolp {
            #[derive(Validate)]
            #[path = $path]
            struct ValidatedJSON;
        }
    }
}
