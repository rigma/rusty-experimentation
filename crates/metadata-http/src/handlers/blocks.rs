use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use metadata_data_layer::{extract::Repository, repositories::BlockRepository};

pub(super) async fn show(
    Path((_, block_name)): Path<(String, String)>,
    Repository(repository): Repository<BlockRepository>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match repository.get_block_by_name(&block_name).await {
        Ok(Some(block)) => Ok(Json(block)),
        // TODO(rigma): improve error handling instead of returning raw responses
        Ok(None) => Err((StatusCode::NOT_FOUND, "Block not found!")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "SQL error!")),
    }
}
