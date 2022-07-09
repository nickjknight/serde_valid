use std::collections::HashMap;

use crate::{
    types::Field,
    validate::{common::get_str, Validator},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_string_pattern_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Result<Validator, crate::Errors> {
    Ok(inner_extract_string_pattern_validator(
        field,
        validation_value,
        message_fn,
        rename_map,
    )?)
}

fn inner_extract_string_pattern_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let rename = rename_map.get(field_name).unwrap_or(field_name);
    let pattern = get_str(validation_value)?;
    let message = message_fn.unwrap_or(quote!(
        ::serde_valid::PatternErrorParams::to_default_message
    ));
    let pattern_ident = syn::Ident::new(
        &format!("{}_PATTERN", &field_ident).to_uppercase(),
        field_ident.span(),
    );

    Ok(quote!(
        static #pattern_ident : ::once_cell::sync::OnceCell<::regex::Regex> = ::once_cell::sync::OnceCell::new();
        let __pattern = #pattern_ident.get_or_init(|| ::regex::Regex::new(#pattern).unwrap());
        if let Err(__multi_error_params) = ::serde_valid::validation::ValidatePatterns::validate_patterns(
            #field_ident,
            __pattern,
        ) {

            use ::serde_valid::error::ToDefaultMessage;
            match __multi_error_params {
                ::serde_valid::validation::Multiple::Single(__single_error_params) => {
                    __properties_errors
                            .entry(#rename)
                            .or_default()
                            .push(::serde_valid::validation::Error::Pattern(
                                ::serde_valid::error::Message::new(
                                    __single_error_params,
                                    #message
                                )
                            ));
                },
                ::serde_valid::validation::Multiple::Array(__vec_error_params) => {
                    __vec_error_params
                        .into_iter()
                        .for_each(|__error_params| {
                            match __error_params {
                                ::serde_valid::validation::Multiple::Single(__single_error_params) => {
                                    __properties_errors
                                        .entry(#rename)
                                        .or_default()
                                        .push(::serde_valid::validation::Error::Pattern(
                                            ::serde_valid::error::Message::new(
                                                __single_error_params,
                                                #message
                                            )
                                        ));
                                }
                                _ => (),
                            }
                        });
                },
            }
        }
    ))
}
