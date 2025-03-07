use crate::serde::rename::RenameMap;
use crate::types::{Field, SingleIdentPath};
use crate::validate::common::{CustomMessageToken, MetaListValidation};
use crate::validate::generic::{
    extract_generic_custom_validator, extract_generic_enumerate_validator,
};
use crate::validate::Validator;
use std::str::FromStr;

pub fn extract_validator_from_nested_meta_list(
    field: &impl Field,
    validation_list: &syn::MetaList,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let syn::MetaList {
        path: validation_name,
        ..
    } = validation_list;
    let validation_ident = SingleIdentPath::new(validation_name).ident();

    match MetaListValidation::from_str(&validation_ident.to_string()) {
        Ok(MetaListValidation::Enumerate) => {
            extract_generic_enumerate_validator(field, validation_list, custom_message, rename_map)
        }
        Ok(MetaListValidation::Custom) => {
            extract_generic_custom_validator(field, validation_list, rename_map)
        }
        Err(unknown) => Err(vec![crate::Error::validate_unknown_type(
            validation_name,
            &unknown,
            &MetaListValidation::iter()
                .map(|x| x.name())
                .collect::<Vec<_>>(),
        )]),
    }
}
