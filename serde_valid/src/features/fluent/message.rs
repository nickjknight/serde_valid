use fluent_0::FluentValue;

use crate::validation::ToDefaultMessage;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: &'static str,
    pub args: Vec<(&'static str, FluentValue<'static>)>,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl ToDefaultMessage for Message {
    #[inline]
    fn to_default_message(&self) -> String {
        self.id.to_string()
    }
}
