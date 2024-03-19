use axum::extract::FromRef;
use metadata_data_layer_utils::PoolState;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub(crate) pool: Arc<PoolState>,
}

impl AppState {
    pub fn new(pool: PoolState) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

impl FromRef<AppState> for Arc<PoolState> {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}
