use serde_json::json;
use serde_valid::{Validate, ValidateFormat};

fn gmail(gmail: &str) -> Result<(), String> {
    if gmail.ends_with("@gmail.com") {
        Ok(())
    } else {
        Err("gmail".to_owned())
    }
}

fn os_str(os_str: &std::ffi::OsStr) -> Result<(), String> {
    if os_str == "os_text" {
        Ok(())
    } else {
        Err("os_str".to_owned())
    }
}

fn path(path: &std::path::Path) -> Result<(), String> {
    if path == std::path::PathBuf::from("/tmp/user.json") {
        Ok(())
    } else {
        Err("path".to_owned())
    }
}

#[test]
fn format_string_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("your@gmail.com"),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn format_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(format(gmail))]
        val: &'a str,
    }

    let s = TestStruct {
        val: "your@gmail.com",
    };

    assert!(s.validate().is_ok());
}

#[test]
fn format_cow_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(format(gmail))]
        val: std::borrow::Cow<'a, str>,
    }

    let s = TestStruct {
        val: std::borrow::Cow::from("your@gmail.com"),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn format_os_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(format(os_str))]
        val: &'a std::ffi::OsStr,
    }

    let s = TestStruct {
        val: std::ffi::OsStr::new("os_text"),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn format_os_string_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(os_str))]
        val: std::ffi::OsString,
    }

    let s = TestStruct {
        val: std::ffi::OsString::from("os_text"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn format_path_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(format(path))]
        val: &'a std::path::Path,
    }

    let s = TestStruct {
        val: std::path::Path::new("/tmp/user.json"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn format_path_buf_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(path))]
        val: std::path::PathBuf,
    }

    let s = TestStruct {
        val: std::path::PathBuf::from("/tmp/user.json"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn format_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("user@outlook.com"),
    };
    assert!(s.validate().is_err());
}

#[test]
fn format_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec![
            String::from("user@gmail.com"),
            String::from("you@gmail.com"),
        ],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn format_nested_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: Vec<Vec<String>>,
    }

    let s = TestStruct {
        val: vec![
            vec![
                String::from("user@gmail.com"),
                String::from("admin@gmail.com"),
            ],
            vec![String::from("me@gmail.com"), String::from("you@gmail.com")],
        ],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn format_option_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: Option<String>,
    }

    let s = TestStruct {
        val: Some(String::from("user@gmail.com")),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn format_nested_option_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: Option<Option<String>>,
    }

    let s = TestStruct {
        val: Some(Some(String::from("user@gmail.com"))),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn format_vec_optional_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: Vec<Option<String>>,
    }

    let s = TestStruct {
        val: vec![
            Some(String::from("user@gmail.com")),
            Some(String::from("admin@gmail.com")),
            None,
        ],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn format_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("user@outlook.com"),
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "The value does not follow the \"gmail\" format."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn format_custom_err_message_fn() {
    fn error_message(_params: &serde_valid::FormatErrorParams) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail), message_fn(error_message))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("user@outlook.com"),
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is custom message."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn format_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail), message = "this is custom message.")]
        val: String,
    }

    let s = TestStruct {
        val: String::from("user@outlook.com"),
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is custom message."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn format_trait() {
    struct MyType(String);

    impl ValidateFormat<str> for MyType {
        fn validate_format<F: FnOnce(&str) -> Result<(), String>>(
            &self,
            validate_fn: F,
        ) -> Result<(), serde_valid::FormatErrorParams> {
            self.0.validate_format(validate_fn)
        }
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(format(gmail))]
        val: MyType,
    }

    let s = TestStruct {
        val: MyType(String::from("user@gmail.com")),
    };

    assert!(s.validate().is_ok());
}
