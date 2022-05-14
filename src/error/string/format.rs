use crate::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct FormatErrorParams {
    format: String,
}

impl FormatErrorParams {
    pub fn new(format: &str) -> Self {
        Self {
            format: format.to_owned(),
        }
    }

    #[allow(dead_code)]
    pub fn format(&self) -> &str {
        &self.format
    }
}

impl ToDefaultMessage for FormatErrorParams {
    fn to_default_message(&self) -> String {
        format!("The value does not follow the \"{}\" format.", self.format)
    }
}
