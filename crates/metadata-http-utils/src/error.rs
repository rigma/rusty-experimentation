use crate::problems::{self, Problem};
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::{header, StatusCode};
use serde_json::json;

#[derive(Debug)]
pub enum HttpError {
    ProblemError(Box<dyn Problem>),
    SQLError(sqlx::Error),
}

impl From<sqlx::Error> for HttpError {
    fn from(value: sqlx::Error) -> Self {
        Self::SQLError(value)
    }
}

impl<P> From<P> for HttpError
where
    P: Problem + 'static,
{
    fn from(value: P) -> Self {
        Self::ProblemError(Box::new(value))
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            Self::ProblemError(problem) => {
                let (ty, title, detail, status, instance, headers) = problem.parts();

                let headers = if let Some(mut headers) = headers {
                    if let Some(content_type) = headers.get_mut(header::CONTENT_TYPE) {
                        *content_type = problems::CONTENT_TYPE;
                    } else {
                        headers.append(header::CONTENT_TYPE, problems::CONTENT_TYPE);
                    }

                    headers
                } else {
                    problems::default_headers()
                };

                let body = Json(match (status, instance) {
                    (Some(status), Some(instance)) => json!({
                        "type": ty,
                        "title": title,
                        "detail": detail,
                        "status": status.as_u16(),
                        "instance": instance,
                    }),
                    (Some(status), None) => json!({
                        "type": ty,
                        "title": title,
                        "detail": detail,
                        "status": status.as_u16(),
                    }),
                    (None, Some(instance)) => json!({
                        "type": ty,
                        "title": title,
                        "detail": detail,
                        "instance": instance,
                    }),
                    _ => json!({
                        "type": ty,
                        "title": title,
                        "detail": detail,
                    }),
                });

                (status.unwrap_or(StatusCode::BAD_REQUEST), headers, body).into_response()
            }
            Self::SQLError(error) => {
                use sqlx::Error;

                let mut headers = problems::default_headers();
                let (status_code, body) = match error {
                    Error::PoolClosed | Error::PoolTimedOut => {
                        // TODO(rigma): arbitrary value used here
                        headers.append(header::RETRY_AFTER, 120.into());

                        (
                            StatusCode::GATEWAY_TIMEOUT,
                            json!({
                                "type": "https://errors.taster.com/metadata/sql/connection-closed",
                                "title": "Database Connection Closed.",
                                "detail": format!("{error}"),
                            }),
                        )
                    }
                    // TODO(rigma): you should support more SQLx errors in the future
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        json!({
                            "type": "https://errors.taster.com/metadata/sql/unknown-error",
                            "title": "Unknown database error",
                            "detail": format!("{error}"),
                        }),
                    ),
                };

                (status_code, headers, Json(body)).into_response()
            }
        }
    }
}
