use crate::error::FormatErrorParams;

/// Format validation of the string.
///
/// See [JsonSchema String Format](https://json-schema.org/understanding-json-schema/reference/string.html#format)
///
/// ```rust
/// use serde_valid::{Validate, ValidateFormat};
///
/// struct MyType(String);
///
/// impl ValidateFormat<str> for MyType {
///     fn validate_format<F: FnOnce(&str) -> Result<(), String>>(
///         &self,
///         validate_fn: F,
///     ) -> Result<(), serde_valid::FormatErrorParams> {
///         self.0.validate_format(validate_fn)
///     }
/// }
///
/// fn gmail(gmail: &str) -> Result<(), String> {
///     if gmail.ends_with("@gmail.com") {
///         Ok(())
///     } else {
///         Err("gmail".to_owned())
///     }
/// }
///
/// #[derive(Validate)]
/// struct SampleStruct {
///     #[validate(format(gmail))]
///     val: MyType,
/// }
///
/// let s = SampleStruct {
///     val: MyType(String::from("user@gmail.com")),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
pub trait ValidateFormat<T: ?Sized> {
    fn validate_format<F: FnOnce(&T) -> Result<(), String>>(
        &self,
        validate_fn: F,
    ) -> Result<(), FormatErrorParams>;
}

macro_rules! impl_validate_string_format {
    ($ty:ty, $ref:ty) => {
        impl ValidateFormat<$ref> for $ty {
            fn validate_format<F: FnOnce(&$ref) -> Result<(), String>>(
                &self,
                validate_fn: F,
            ) -> Result<(), FormatErrorParams> {
                match validate_fn(self) {
                    Ok(_) => Ok(()),
                    Err(format) => Err(FormatErrorParams::new(format)),
                }
            }
        }
    };
}

impl_validate_string_format!(str, str);
impl_validate_string_format!(&str, str);
impl_validate_string_format!(String, str);
impl_validate_string_format!(std::borrow::Cow<'_, str>, str);
impl_validate_string_format!(std::ffi::OsStr, std::ffi::OsStr);
impl_validate_string_format!(&std::ffi::OsStr, std::ffi::OsStr);
impl_validate_string_format!(std::ffi::OsString, std::ffi::OsStr);
impl_validate_string_format!(std::borrow::Cow<'_, std::ffi::OsStr>, std::ffi::OsStr);
impl_validate_string_format!(std::path::Path, std::path::Path);
impl_validate_string_format!(&std::path::Path, std::path::Path);
impl_validate_string_format!(std::path::PathBuf, std::path::Path);
impl_validate_string_format!(std::borrow::Cow<'_, std::path::Path>, std::path::Path);

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_gmail(gmail: &str) -> Result<(), String> {
        if gmail.ends_with("@gmail.co.jp") {
            Ok(())
        } else {
            Err("gmail".to_string())
        }
    }

    #[test]
    fn test_validate_string_format_str_type() {
        assert!("user@gmail.co.jp".validate_format(validate_gmail).is_ok());
    }

    #[test]
    fn test_validate_string_format_ref_str_type() {
        assert!((&"user@gmail.co.jp")
            .validate_format(validate_gmail)
            .is_ok());
    }

    #[test]
    fn test_validate_string_format_string_type() {
        assert!(String::from("user@gmail.co.jp")
            .validate_format(validate_gmail)
            .is_ok());
    }

    #[test]
    fn test_validate_string_format_cow_str_type() {
        assert!(std::borrow::Cow::from("user@gmail.co.jp")
            .validate_format(validate_gmail)
            .is_ok());
    }
}
