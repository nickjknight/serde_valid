use std::collections::HashMap;

use crate::types::Field;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_generic_custom_validator(
    field: &impl Field,
    syn::MetaList { path, nested, .. }: &syn::MetaList,
    rename_map: &HashMap<String, String>,
) -> Result<Validator, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let rename = rename_map.get(field_name).unwrap_or(field_name);

    let custom_fn_name = match nested.len() {
        0 => Err(vec![crate::Error::validate_custom_need_item(path)]),
        1 => extract_custom_fn_name(&nested[0]).map_err(|error| vec![error]),
        _ => Err(nested
            .iter()
            .skip(1)
            .map(|arg| crate::Error::validate_custom_tail_error(arg))
            .collect()),
    }?;

    Ok(Validator::Normal(quote!(
        if let Err(__error) = #custom_fn_name(#field_ident) {
            __errors
                .entry(#rename)
                .or_default()
                .push(__error);
        };
    )))
}

fn extract_custom_fn_name(nested_meta: &syn::NestedMeta) -> Result<TokenStream, crate::Error> {
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
