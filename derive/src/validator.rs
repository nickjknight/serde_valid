mod array;
mod generic;
mod meta;
mod number;
mod object;
mod string;

use crate::abort::abort_invalid_attribute_on_field;
use crate::helper::NamedField;
use meta::extract_meta_validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use ref_cast::RefCast;
use std::iter::FromIterator;
use syn::parse_quote;
use syn::spanned::Spanned;

pub enum Validator {
    Normal(TokenStream),
    Option(Box<Validator>),
    Array(Box<Validator>),
}

pub struct FieldValidators {
    field: NamedField,
    validators: Vec<TokenStream>,
    optional_validators: Option<Box<FieldValidators>>,
    array_validators: Option<Box<FieldValidators>>,
}

impl FieldValidators {
    pub fn new(field: syn::Field) -> Self {
        Self::inner_new(NamedField::new(field))
    }

    fn inner_new(field: NamedField) -> Self {
        Self {
            field,
            validators: vec![],
            optional_validators: None,
            array_validators: None,
        }
    }

    pub fn push(&mut self, validator: Validator) {
        match validator {
            Validator::Normal(token) => self.validators.push(token),
            Validator::Option(ty) => match self.optional_validators.as_mut() {
                Some(optional_validator) => optional_validator.push(*ty),
                None => {
                    if let Some(field) = self.field.option_field() {
                        let mut option_validators = Box::new(Self::inner_new(field));
                        option_validators.push(*ty);
                        self.optional_validators = Some(option_validators);
                    }
                }
            },
            Validator::Array(ty) => match self.array_validators.as_mut() {
                Some(array_validator) => array_validator.push(*ty),
                None => {
                    if let Some(field) = self.field.array_field() {
                        let mut array_validators = Box::new(Self::inner_new(field));
                        array_validators.push(*ty);
                        self.array_validators = Some(array_validators);
                    }
                }
            },
        }
    }

    pub fn to_token(&self) -> TokenStream {
        let ident = self.field.ident();

        // Nomal Tokens
        let normal_tokens = if !self.validators.is_empty() {
            let validators = TokenStream::from_iter(self.validators.clone());
            quote! (#validators)
        } else {
            quote! {}
        };

        // Optional Tokens
        let optional_tokens = if let Some(optional_validators) = &self.optional_validators {
            let option_ident = optional_validators.field.ident();
            let option_validators = optional_validators.to_token();
            quote!(
                if let Some(#option_ident) = #ident {
                    #option_validators
                }
            )
        } else {
            quote!()
        };

        // Array Tokens
        let array_tokens = if let Some(array_validators) = &self.array_validators {
            let array_ident = array_validators.field.ident();
            let array_validators = array_validators.to_token();
            quote!(
                for #array_ident in #ident {
                    #array_validators
                }
            )
        } else {
            quote!()
        };

        quote!(
            #normal_tokens
            #optional_tokens
            #array_tokens
        )
    }

    pub fn generate_token(&self) -> TokenStream {
        let field_ident = self.field.ident();
        let validation = self.to_token();
        quote!(
            let #field_ident = &self.#field_ident;
            #validation
        )
    }
}

/// Find the types (as string) for each field of the struct
/// Needed for the `must_match` filter
pub fn collect_validators(fields: &syn::Fields) -> Vec<FieldValidators> {
    let mut struct_validators = vec![];
    for field in fields {
        let mut field_validators = FieldValidators::new(field.clone());
        let named_field = NamedField::ref_cast(field);
        let field_ident = named_field.ident();
        for attribute in named_field.attrs() {
            if attribute.path != parse_quote!(validate) {
                continue;
            }
            let validator = extract_meta_validator(&named_field, attribute);
            match validator {
                Some(validator) => field_validators.push(validator),
                None => abort_invalid_attribute_on_field(
                    &field_ident,
                    attribute.span(),
                    "it needs at least one validator",
                ),
            }
        }
        struct_validators.push(field_validators)
    }

    struct_validators
}

#[allow(dead_code)]
fn get_field_type(field_type: &syn::Type, field_ident: &syn::Ident) -> String {
    match field_type {
        syn::Type::Path(syn::TypePath { ref path, .. }) => path.to_token_stream().to_string(),
        syn::Type::Reference(syn::TypeReference {
            ref lifetime,
            ref elem,
            ..
        }) => {
            if lifetime.is_some() {
                format!("&{}", elem.to_token_stream())
            } else {
                elem.to_token_stream().to_string()
            }
        }
        _ => {
            abort!(
                field_type.span(),
                "Type `{}` of field `{}` not supported",
                field_type.to_token_stream(),
                field_ident
            )
        }
    }
}

#[allow(dead_code)]
fn find_original_field_name<'a>(meta_items: &[&'a syn::NestedMeta]) -> Option<&'a syn::LitStr> {
    for meta_item in meta_items {
        match **meta_item {
            syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::Path(_) => continue,
                syn::Meta::NameValue(syn::MetaNameValue {
                    ref path, ref lit, ..
                }) => {
                    let ident = path.get_ident().unwrap();
                    if ident == "rename" {
                        if let syn::Lit::Str(lit_str) = lit {
                            return Some(lit_str);
                        }
                    }
                }
                syn::Meta::List(syn::MetaList { ref nested, .. }) => {
                    return find_original_field_name(&nested.iter().collect::<Vec<_>>());
                }
            },
            _ => unreachable!(),
        };
    }
    None
}
