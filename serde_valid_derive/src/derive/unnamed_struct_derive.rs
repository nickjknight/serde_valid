use crate::error::{array_errors_tokens, new_type_errors_tokens};
use crate::rule::collect_rules_from_unnamed_struct;
use crate::types::{Field, UnnamedField};
use crate::validate::{extract_meta_validator, FieldValidators};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use syn::parse_quote;

pub fn expand_unnamed_struct_derive(
    input: &syn::DeriveInput,
    fields: &syn::FieldsUnnamed,
) -> Result<TokenStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut errors = vec![];

    let (rule_fields, rules) = match collect_rules_from_unnamed_struct(&input.attrs) {
        Ok((rule_fields, rules)) => (rule_fields, TokenStream::from_iter(rules)),
        Err(rule_errors) => {
            errors.extend(rule_errors);
            (HashSet::new(), quote!())
        }
    };

    let validates = match collect_unnamed_fields_validators_list(fields) {
        Ok(field_validators) => TokenStream::from_iter(field_validators.iter().map(|validator| {
            if validator.is_empty() && rule_fields.contains(validator.ident()) {
                validator.get_field_variable_token()
            } else {
                validator.generate_tokens()
            }
        })),
        Err(validation_errors) => {
            errors.extend(validation_errors.into_iter());
            quote!()
        }
    };

    let fields_errors = if fields.unnamed.len() != 1 {
        array_errors_tokens()
    } else {
        new_type_errors_tokens()
    };

    if errors.is_empty() {
        Ok(quote!(
            impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
                fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
                    let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                    let mut __item_vec_errors_map = ::serde_valid::validation::ItemVecErrorsMap::new();

                    #validates
                    #rules

                    if __rule_vec_errors.is_empty() && __item_vec_errors_map.is_empty() {
                        Ok(())
                    } else {
                        Err(#fields_errors)
                    }
                }
            }
        ))
    } else {
        Err(errors)
    }
}

pub fn collect_unnamed_fields_validators_list(
    fields: &syn::FieldsUnnamed,
) -> Result<Vec<FieldValidators<UnnamedField>>, crate::Errors> {
    let mut errors = vec![];

    let validators = fields
        .unnamed
        .iter()
        .enumerate()
        .filter_map(|field| match collect_unnamed_field_validators(field) {
            Ok(validators) => Some(validators),
            Err(ref mut error) => {
                errors.append(error);
                None
            }
        })
        .collect();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(validators)
}

fn collect_unnamed_field_validators(
    (index, field): (usize, &syn::Field),
) -> Result<FieldValidators<UnnamedField>, crate::Errors> {
    let mut errors = vec![];

    let unnamed_field = UnnamedField::new(index, field);

    let validators = unnamed_field
        .attrs()
        .iter()
        .filter_map(|attribute| {
            if attribute.path == parse_quote!(validate)
                || attribute.path == parse_quote!(serde_valid)
            {
                match extract_meta_validator(&unnamed_field, attribute, &HashMap::new()) {
                    Ok(validator) => Some(validator),
                    Err(validator_errors) => {
                        errors.extend(validator_errors);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(FieldValidators::new(Cow::Owned(unnamed_field), validators))
}
