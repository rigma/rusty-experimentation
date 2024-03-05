use super::{repository, state::PoolState};
use async_trait::async_trait;
use axum_core::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use std::convert::Infallible;

#[derive(Clone, Debug)]
pub struct Repository<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Repository<T>
where
    PoolState: FromRef<S>,
    T: repository::Repository,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // TODO(rigma): avoid to panic here and use a custom error.
        Ok(Self(T::from_connection(
            PoolState::from_ref(state)
                .begin_connection()
                .await
                .expect("A connection cannot be established with the database"),
        )))
    }
}
