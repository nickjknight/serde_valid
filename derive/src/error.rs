use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn fields_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Fields(__errors))
}

pub fn new_type_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::NewType(
        __errors.remove("0").unwrap()
    ))
}

#[derive(Debug)]
pub struct Error(syn::Error);

impl Error {
    fn new<Message: Into<String>>(span: proc_macro2::Span, message: Message) -> Self {
        Self(syn::Error::new(span, message.into()))
    }

    pub fn unit_struct_not_support(input: &syn::DeriveInput) -> Self {
        Self::new(
            input.span(),
            "#[derive(Validate)] does not support Unit Struct.",
        )
    }

    pub fn union_not_support(input: &syn::DeriveInput) -> Self {
        Self::new(input.span(), "#[derive(Validate)] does not support Union.")
    }

    pub fn rule_need_function(path: &syn::Path) -> Self {
        Self::new(path.span(), "#[rule(???)] needs rule_fn.")
    }

    pub fn rule_allow_single_function(nested_meta: &syn::NestedMeta) -> Self {
        Self::new(nested_meta.span(), "#[rule] allow single function.")
    }

    pub fn rule_need_arguments(path: &syn::Path) -> Self {
        Self::new(path.span(), "`rule` function needs arguments.")
    }

    pub fn rule_allow_path_arguments(
        rule_fn_name_path: &syn::Path,
        meta: &syn::NestedMeta,
    ) -> Self {
        let rule_fn_name = quote!(#rule_fn_name_path).to_string();
        Self::new(
            meta.span(),
            format!("#[rule({rule_fn_name}(???, ...))] allow field path only."),
        )
    }

    pub fn rule_allow_index_arguments(
        rule_fn_name_path: &syn::Path,
        meta: &syn::NestedMeta,
    ) -> Self {
        let rule_fn_name = quote!(#rule_fn_name_path).to_string();
        Self::new(
            meta.span(),
            format!("#[rule({rule_fn_name}(???, ...))] allow index integer only."),
        )
    }

    pub fn rule_validate_attribute_parse_error(
        attribute: &syn::Attribute,
        error: &syn::Error,
    ) -> Self {
        Self::new(attribute.span(), format!("#[rule] parse error: {error}"))
    }

    pub fn validate_meta_literal_not_support(lit: &syn::Lit) -> Self {
        Self::new(lit.span(), "#[validate(???)] does not support literal.")
    }

    pub fn validate_meta_name_value_not_support(name_value: syn::MetaNameValue) -> Self {
        Self::new(
            name_value.span(),
            "#[validate = ???] format does not support.",
        )
    }

    pub fn validate_meta_name_value_need_value(path: &syn::Path, validation_type: &str) -> Self {
        Self::new(
            path.span(),
            format!("#[validate({validation_type} = ???)] needs validation value."),
        )
    }

    pub fn validate_meta_path_need_value(path: &syn::Path, validation_type: &str) -> Self {
        Self::new(
            path.span(),
            format!("#[validate({validation_type}(???))] needs validation path."),
        )
    }

    pub fn validate_meta_list_need_value(path: &syn::Path, validation_type: &str) -> Self {
        Self::new(
            path.span(),
            format!("#[validate({validation_type}(???, ...))] needs validation list."),
        )
    }

    pub fn validate_attribute_parse_error(attribute: &syn::Attribute, error: &syn::Error) -> Self {
        Self::new(
            attribute.span(),
            format!("#[validate] parse error: {error}"),
        )
    }

    pub fn validate_need_type(attribute: &syn::Attribute) -> Self {
        Self::new(
            attribute.path.span(),
            "#[validate(???)] needs validation type.",
        )
    }

    pub fn validate_unknown_type(path: &syn::Path, unknown: &str, candidates: &[&str]) -> Self {
        let filterd_candidates = did_you_mean(unknown, candidates).unwrap_or(candidates.to_vec());

        Self::new(
            path.span(),
            format!("Unknown: `{unknown}`. Is it one of the following?\n{filterd_candidates:#?}"),
        )
    }

    pub fn validate_format_need_item(path: &syn::Path) -> Self {
        Self::new(
            path.span(),
            format!("#[validate(format(???))] need format_fn."),
        )
    }

    pub fn validate_format_tail_error(nested_meta: &syn::NestedMeta) -> Self {
        Self::new(
            nested_meta.span(),
            format!("#[validate(format(???))] support only 1 item."),
        )
    }

    pub fn validate_enumerate_need_item(path: &syn::Path) -> Self {
        Self::new(
            path.span(),
            format!("#[validate(enumerate(???, ...))] need items."),
        )
    }

    pub fn validate_custom_need_item(path: &syn::Path) -> Self {
        Self::new(
            path.span(),
            format!("#[validate(custom(???))] need custom_fn."),
        )
    }

    pub fn validate_custom_tail_error(nested_meta: &syn::NestedMeta) -> Self {
        Self::new(
            nested_meta.span(),
            format!("#[validate(custom(???))] support only 1 item."),
        )
    }

    pub fn message_fn_need_item(path: &syn::Path) -> Self {
        Self::new(
            path.span(),
            format!("#[validate(..., message_fn(???))] need items."),
        )
    }

    pub fn message_fn_allow_name_path(nested_meta: &syn::NestedMeta) -> Self {
        Self::new(
            nested_meta.span(),
            format!("#[validate(..., message_fn(???))] allow only function name path."),
        )
    }

    pub fn message_fn_tail_error(nested_meta: &syn::NestedMeta) -> Self {
        Self::new(
            nested_meta.span(),
            format!("#[validate(..., message_fn(???))] support only 1 item."),
        )
    }

    pub fn literal_only(meta: &syn::Meta) -> Self {
        Self::new(meta.span(), "Allow literal only.")
    }

    pub fn numeric_literal_only(lit: &syn::Lit) -> Self {
        Self::new(lit.span(), "Allow numeric literal only.")
    }

    pub fn str_literal_only(lit: &syn::Lit) -> Self {
        Self::new(lit.span(), "Allow str literal only.")
    }

    pub fn literal_not_support(lit: &syn::Lit) -> Self {
        Self::new(lit.span(), "Literal does not support.")
    }

    pub fn meta_name_value_not_support(name_value: &syn::MetaNameValue) -> Self {
        Self::new(name_value.span(), "Name value does not support.")
    }

    pub fn meta_path_not_support(path: &syn::Path) -> Self {
        Self::new(path.span(), "Path does not support.")
    }

    pub fn too_many_list_items(nested_meta: &syn::NestedMeta) -> Self {
        Self::new(nested_meta.span(), "Too many list items.")
    }

    pub fn to_compile_error(&self) -> TokenStream {
        self.0.to_compile_error()
    }
}

fn did_you_mean<'a, T, I>(unknown: &'a str, candidates: I) -> Option<Vec<&'a str>>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let mut filterd = candidates
        .into_iter()
        .map(|candidate| {
            (
                ::strsim::jaro_winkler(unknown, candidate.as_ref()),
                candidate.as_ref(),
            )
        })
        .filter(|(confidence, _)| *confidence > 0.8)
        .collect::<Vec<_>>();

    if filterd.len() == 0 {
        None
    } else {
        filterd.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        Some(
            filterd
                .into_iter()
                .map(|(_, candidate)| candidate)
                .collect(),
        )
    }
}

pub type Errors = Vec<Error>;

pub fn to_compile_errors(errors: Errors) -> TokenStream {
    let compile_errors = errors.iter().map(Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
