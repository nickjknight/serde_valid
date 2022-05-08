mod deserialize;
pub mod error;
mod traits;
pub mod validation;

pub use deserialize::*;
pub use error::{
    EnumerateErrorParams, Error, ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams,
    MaxItemsErrorParams, MaxLengthErrorParams, MaxPropertiesErrorParams, MaximumErrorParams,
    MinItemsErrorParams, MinLengthErrorParams, MinPropertiesErrorParams, MinimumErrorParams,
    MultipleOfErrorParams, PatternErrorParams, UniqueItemsErrorParams,
};
pub use traits::*;
pub use validation::{
    validate_array_max_items, validate_array_min_items, validate_array_unique_items,
    validate_generic_enumerate, validate_object_max_properties, validate_object_min_properties,
    validate_string_max_length, validate_string_min_length, validate_string_pattern,
    ValidateNumericExclusiveMaximum, ValidateNumericExclusiveMinimum, ValidateNumericMaximum,
    ValidateNumericMinimum, ValidateNumericMultipleOf,
};

pub trait Validate {
    fn validate(&self) -> std::result::Result<(), self::validation::Errors>;
}

pub use serde_valid_derive::Validate;
