#[warn(clippy::needless_collect)]
mod derive;
mod error;
mod rule;
mod serde;
mod types;
mod validate;

use derive::expand_derive;
use error::to_compile_errors;
use error::{Error, Errors};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Validate, attributes(rule, validate, serde_valid))]
#[proc_macro_error]
pub fn derive_validate(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    expand_derive(&input)
        .unwrap_or_else(to_compile_errors)
        .into()
}
