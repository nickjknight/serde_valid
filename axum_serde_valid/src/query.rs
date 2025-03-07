#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! A simple crate provides a drop-in replacement for [`axum::extract::Query`]
//! that uses [`jsonschema`] to validate requests schemas
//! generated via [`schemars`].
//!
//! You might want to do this in order to provide a better
//! experience for your clients and not leak serde's error messages.
//!
//! All schemas are cached in a thread-local storage for
//! the life of the application (or thread).
//!
//! # Features
//!
//! - aide: support for [aide](https://docs.rs/aide/latest/aide/)

use std::ops::Deref;

use async_trait::async_trait;
use axum::http::Request;
use axum::{extract::FromRequest, BoxError};

/// Wrapper type over [`axum::extract::Query`] that validates
/// requests with a more helpful validation
/// message.
pub struct Query<T>(pub T);

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Query<T> {
    fn from(data: T) -> Self {
        Query(data)
    }
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Query<T>
where
    B: http_body::Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
    T: crate::validated::Deserialize + 'static,
{
    type Rejection = crate::rejection::Rejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        crate::request::from_request::<_, _, T>(req, state)
            .await
            .map(Query)
    }
}

#[cfg(feature = "aide")]
mod impl_aide {
    use super::*;

    impl<T> aide::OperationInput for Query<T>
    where
        T: schemars::JsonSchema,
    {
        fn operation_input(
            ctx: &mut aide::gen::GenContext,
            operation: &mut aide::openapi::Operation,
        ) {
            axum::extract::Query::<T>::operation_input(ctx, operation);
        }
    }
}
