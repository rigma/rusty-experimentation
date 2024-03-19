use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use metadata_data_layer::repositories::DomainRepository;
use metadata_data_layer_utils::extract::Repository;
use metadata_http_utils::{HttpError, Problem};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
enum DomainError {
    #[error("Domain '{0}' is not found.")]
    NotFoundByName(String),
}

impl Problem for DomainError {
    fn ty(&self) -> String {
        let sub_type = match self {
            Self::NotFoundByName(_) => "not-found",
        };

        format!("https://errors.taster.com/metadata/domains/{sub_type}")
    }

    fn title(&self) -> String {
        match self {
            Self::NotFoundByName(_) => "Domain Not Found.".to_string(),
        }
    }

    fn detail(&self) -> String {
        format!("{self}")
    }

    fn status(&self) -> Option<StatusCode> {
        match self {
            Self::NotFoundByName(_) => Some(StatusCode::NOT_FOUND),
        }
    }
}

pub(super) async fn show(
    Path(domain_name): Path<String>,
    Repository(repository): Repository<DomainRepository>,
) -> Result<impl IntoResponse, HttpError> {
    let domain = repository.get_domain_by_name(&domain_name).await?;
    if let Some(domain) = domain {
        Ok(Json(domain))
    } else {
        Err(DomainError::NotFoundByName(domain_name).into())
    }
}
