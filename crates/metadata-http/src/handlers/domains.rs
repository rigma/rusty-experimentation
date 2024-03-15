use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use metadata_data_layer::{extract::Repository, repositories::DomainRepository};

pub(super) async fn show(
    Path(domain_name): Path<String>,
    Repository(repository): Repository<DomainRepository>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match repository.get_domain_by_name(&domain_name).await {
        Ok(Some(domain)) => Ok(Json(domain)),
        // TODO(rigma): improve error handling instead of returning raw responses
        Ok(None) => Err((StatusCode::NOT_FOUND, "Domain not found!")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "SQL error!")),
    }
}
