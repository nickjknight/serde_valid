mod lit;
mod message;

pub use lit::{get_numeric, get_str};
pub use message::extract_message_fn_tokens;

macro_rules! count {
    () => (0usize);
    ( $x:literal $($xs:literal)* ) => (1usize + count!($($xs)*));
}

macro_rules! enum_str {
    (pub enum $name:ident {}) => {
        pub enum $name {
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                unimplemented!()
            }

            pub fn iter() -> std::array::IntoIter<Self, 0> {
                [].into_iter()
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Err(s.to_owned())
            }
        }
    };

    (pub enum $name:ident {
        $($variant:ident = $val:literal),*,
    }) => {
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match *self {
                    $($name::$variant => $val),*
                }
            }

            pub fn iter() -> std::array::IntoIter<Self, {count!($($val)*)} > {
                [
                    $($name::$variant),*
                ].into_iter()
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($val => Ok($name::$variant) ),*,
                    _ => Err(s.to_owned())
                }
            }
        }
    };
}

enum_str! {
    pub enum MetaListValidation {
        Format = "format",
        Enumerate = "enumerate",
        Custom = "custom",
    }
}

enum_str! {
    pub enum MetaNameValueValidation {
        Minimum = "minimum",
        Maximum = "maximum",
        ExclusiveMinimum = "exclusive_minimum",
        ExclusiveMaximum = "exclusive_maximum",
        MinLength = "min_length",
        MaxLength = "max_length",
        MinItems = "min_items",
        MaxItems = "max_items",
        MinProperties = "min_properties",
        MaxProperties = "max_properties",
        MultipleOf = "multiple_of",
        Pattern = "pattern",
    }
}

enum_str! {
    pub enum MetaPathValidation {
        UniqueItems = "unique_items",
    }
}

enum_str! {
    pub enum MetaListMessage {
        MessageFn = "message_fn",
    }
}

enum_str! {
    pub enum MetaNameValueMessage {
        Message = "message",
    }
}

enum_str! {
    pub enum MetaPathMessage {
    }
}
