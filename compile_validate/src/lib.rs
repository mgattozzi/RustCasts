#[macro_export]
macro_rules! validate_json {
    ($path: expr) => {
        #[macro_use] extern crate validate_macros;
        #[allow(dead_code)]
        mod dsjfkdsjfolp {
            #![allow(dead_code)]
            #[derive(Validate)]
            #[ValidatePath = $path]
            struct ValidatedJSON;
        }
    }
}
