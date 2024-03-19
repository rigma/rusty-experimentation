use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use metadata_data_layer::repositories::BlockRepository;
use metadata_data_layer_utils::extract::Repository;
use metadata_http_utils::{HttpError, Problem};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum BlockError {
    #[error("Block '{0}' is not found.")]
    NotFoundByName(String),
}

impl Problem for BlockError {
    fn ty(&self) -> String {
        let sub_type = match self {
            Self::NotFoundByName(_) => "not-found",
        };

        format!("https://errors.taster.com/metadata/blocks/{sub_type}")
    }

    fn title(&self) -> String {
        match self {
            Self::NotFoundByName(_) => "Block Not Found.".to_string(),
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
    Path((_, block_name)): Path<(String, String)>,
    Repository(repository): Repository<BlockRepository>,
) -> Result<impl IntoResponse, HttpError> {
    let block = repository.get_block_by_name(&block_name).await?;
    if let Some(block) = block {
        Ok(Json(block))
    } else {
        Err(BlockError::NotFoundByName(block_name).into())
    }
}
