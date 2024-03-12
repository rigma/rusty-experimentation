use super::{repository, state::PoolState};
use async_trait::async_trait;
use axum_core::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use sqlx::postgres::Postgres;
use std::convert::Infallible;

#[derive(Clone, Debug)]
pub struct Repository<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Repository<T>
where
    PoolState: FromRef<S>,
    T: repository::Repository<DB = Postgres>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool_state = PoolState::from_ref(state);

        Ok(Self(T::from_pool(pool_state.get_ref())))
    }
}
