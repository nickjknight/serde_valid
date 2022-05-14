use std::collections::HashMap;

use crate::types::Field;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_string_format_validator(
    field: &impl Field,
    meta_list: &syn::MetaList,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Result<Validator, crate::Errors> {
    if let Some(array_field) = field.array_field() {
        Ok(Validator::Array(Box::new(extract_string_format_validator(
            &array_field,
            meta_list,
            message_fn,
            rename_map,
        )?)))
    } else if let Some(option_field) = field.option_field() {
        Ok(Validator::Option(Box::new(
            extract_string_format_validator(&option_field, meta_list, message_fn, rename_map)?,
        )))
    } else {
        Ok(Validator::Normal(inner_extract_string_format_validator(
            field, meta_list, message_fn, rename_map,
        )?))
    }
}

fn inner_extract_string_format_validator(
    field: &impl Field,
    syn::MetaList { path, nested, .. }: &syn::MetaList,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let rename = rename_map.get(field_name).unwrap_or(field_name);
    let format_fn_name = match nested.len() {
        0 => Err(vec![crate::Error::validate_format_need_item(path)]),
        1 => extract_format_fn_name(&nested[0]).map_err(|error| vec![error]),
        _ => Err(nested
            .iter()
            .skip(1)
            .map(|arg| crate::Error::validate_format_tail_error(arg))
            .collect()),
    }?;
    let message =
        message_fn.unwrap_or(quote!(::serde_valid::FormatErrorParams::to_default_message));

    Ok(quote!(
        if let Err(error_params) = ::serde_valid::ValidateFormat::<_>::validate_format(#field_ident, #format_fn_name) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#rename)
                .or_default()
                .push(::serde_valid::validation::Error::Format(
                    ::serde_valid::error::Message::new(
                        error_params,
                        #message
                    )
                ));
        };
    ))
}

fn extract_format_fn_name(nested_meta: &syn::NestedMeta) -> Result<TokenStream, crate::Error> {
    match nested_meta {
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::List(list) => {
                let fn_name = &list.path;
                Ok(quote!(#fn_name))
            }
            syn::Meta::NameValue(name_value) => {
                Err(crate::Error::meta_name_value_not_support(name_value))?
            }
            syn::Meta::Path(fn_name) => Ok(quote!(#fn_name)),
        },
        syn::NestedMeta::Lit(lit) => Err(crate::Error::literal_not_support(lit))?,
    }
}
