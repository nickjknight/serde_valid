use super::{ArrayErrors, ObjectErrors, VecErrors};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Errors<Err = crate::validation::Error> {
    Array(ArrayErrors<Err>),
    Object(ObjectErrors<Err>),
    NewType(VecErrors<Err>),
}

impl<Err> serde::Serialize for Errors<Err>
where
    Err: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Array(a) => serde::Serialize::serialize(a, serializer),
            Self::Object(o) => serde::Serialize::serialize(o, serializer),
            Self::NewType(n) => {
                #[derive(Debug, Clone, serde::Serialize)]
                struct NewTypeErrors<'a, Err> {
                    errors: &'a VecErrors<Err>,
                }

                serde::Serialize::serialize(&NewTypeErrors { errors: n }, serializer)
            }
        }
    }
}

impl<Err> Errors<Err>
where
    Err: Clone,
{
    pub fn merge(&mut self, other: Errors<Err>) {
        match self {
            Errors::Array(a) => match other {
                Errors::Array(b) => {
                    a.errors.extend(b.errors);

                    for (index, item) in b.items {
                        match a.items.get_mut(&index) {
                            Some(errors) => errors.merge(item),
                            None => {
                                a.items.insert(index, item);
                            }
                        };
                    }
                }
                Errors::Object(_) => {
                    unreachable!("conflict Array and Object in serde_valid::validation::Errors")
                }
                Errors::NewType(errors) => {
                    a.errors.extend(errors.into_iter());
                }
            },
            Errors::NewType(a) => match other {
                Errors::Array(b) => {
                    a.extend(b.errors);
                    *self = Errors::Array(ArrayErrors::new(a.to_vec(), b.items));
                }
                Errors::Object(_) => {
                    unreachable!("conflict Array and Object in serde_valid::validation::Errors")
                }
                Errors::NewType(b) => {
                    a.extend(b);
                }
            },
            Errors::Object(_) => {
                unimplemented!("Object not support yet.")
            }
        }
    }
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(errors) => std::fmt::Display::fmt(errors, f),
            Self::Object(errors) => std::fmt::Display::fmt(errors, f),
            Self::NewType(vec_errors) => {
                let errors = &vec_errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>();
                let value = serde_json::json!({ "errors": errors });
                std::fmt::Display::fmt(&value, f)
            }
        }
    }
}
