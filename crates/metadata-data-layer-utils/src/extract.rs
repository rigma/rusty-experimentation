use super::{repository, PoolState};
use async_trait::async_trait;
use axum_core::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use sqlx::postgres::Postgres;
use std::{convert::Infallible, sync::Arc};

/// An [axum_core] extractor that is instanciating a structure
/// implementing [repository::Repository] from an application
/// state that is able to provide an atomic reference to a
/// [PoolState] structure.
///
/// # Example usage in an application
///
/// ```ignore
/// # use axum::{extract::FromRef, response::IntoResponse, routing::get, Router};
/// # use metadata_data_layer_utils::PoolState;
/// # use std::sync::Arc;
///
/// struct AppState {
///     pool: Arc<PoolState>,
/// }
///
/// # impl FromRef<AppState> for Arc<PoolState> {
/// #   fn from_ref(input: &AppState) -> Self {
/// #       Arc::clone(&input.pool)
/// #   }
/// # }
///
/// struct FooRepository {
///     pool: Arc<Pool<Postgres>>,
/// }
///
/// impl Repository for FooRepository {
///     type DB = Postgres;
///
///     fn from_ref(pool: Arc<Pool<Self::DB>>) -> Self {
///         Self { pool }
///     }
/// }
///
/// fn list_foo(
///     Repository(repo): Repository<FooRepository>,
/// ) -> impl IntoResponse {
///     // Handler body
///     # unimplemented!();
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// let app = Router::new()
///     .route("/foo", get(list_foo))
///     .with_state(AppState { pool: Arc::new(PoolState::from_env()) });
/// # let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
/// #
/// # axum::serve(listener, app).await.unwrap();
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Repository<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Repository<T>
where
    Arc<PoolState>: FromRef<S>,
    T: repository::Repository<DB = Postgres>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool_state = Arc::from_ref(state);

        Ok(Self(T::from_ref(pool_state.downcast_ref())))
    }
}
