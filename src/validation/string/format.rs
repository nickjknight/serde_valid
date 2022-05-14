use crate::error::FormatErrorParams;

/// Format validation of the string.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#format>
pub trait ValidateFormat {
    fn validate_format(&self, format: &str) -> Result<(), FormatErrorParams>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_format_str_type() {
        assert!(ValidateFormat::validate_format(
            "2020-09-10",
            &str::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_format_string_type() {
        assert!(ValidateFormat::validate_format(
            &String::from("2020-09-10"),
            &str::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_format_cow_str_type() {
        assert!(ValidateFormat::validate_format(
            &Cow::from("2020-09-10"),
            &str::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_format_os_str_type() {
        assert!(ValidateFormat::validate_format(
            OsStr::new("2020-09-10"),
            &str::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_format_os_string_type() {
        assert!(ValidateFormat::validate_format(
            &OsString::from("2020-09-10"),
            &str::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_format_path_type() {
        assert!(ValidateFormat::validate_format(
            Path::new("./foo/bar.txt"),
            &str::new(r"^*.txt$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_format_path_buf_type() {
        assert!(ValidateFormat::validate_format(
            &PathBuf::from("./foo/bar.txt"),
            &str::new(r"^*.txt$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_format_is_false() {
        assert!(ValidateFormat::validate_format(
            "2020/09/10",
            &str::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_err());
    }
}
