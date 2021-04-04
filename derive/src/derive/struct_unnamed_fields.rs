use crate::abort::abort_invalid_attribute_on_field;
use crate::types::{Field, UnnamedField};
use crate::validator::{extract_meta_validator, FieldValidators};
use proc_macro2::TokenStream;
use std::iter::FromIterator;
use syn::parse_quote;
use syn::spanned::Spanned;

pub fn expand_struct_unnamed_fields_validators_tokens(fields: &syn::FieldsUnnamed) -> TokenStream {
    TokenStream::from_iter(
        collect_struct_unnamed_fields_validators(fields)
            .iter()
            .map(|validator| validator.generate_tokens()),
    )
}

pub fn collect_struct_unnamed_fields_validators(
    fields: &syn::FieldsUnnamed,
) -> Vec<FieldValidators<UnnamedField>> {
    let mut struct_validators = vec![];
    for (index, field) in fields.unnamed.iter().enumerate() {
        let unnamed_field = UnnamedField::new(index, field.to_owned());
        let mut field_validators = FieldValidators::new(unnamed_field.clone());
        let field_ident = unnamed_field.ident();
        for attribute in unnamed_field.attrs() {
            if attribute.path != parse_quote!(validate) {
                continue;
            }
            let validator = extract_meta_validator(&unnamed_field, attribute);
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
