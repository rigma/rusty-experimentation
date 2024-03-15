use crate::AppState;
use axum::{routing::get, Router};

mod blocks;
mod domains;

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/:domain_name", get(domains::show))
        .route("/:domain_name/:block_name", get(blocks::show))
        .with_state(state)
}
