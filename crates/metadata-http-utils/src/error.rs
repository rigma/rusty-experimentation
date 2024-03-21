use crate::problems::{self, Problem};
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::{header, StatusCode};
use serde_json::json;

/// A generic HTTP error that can be emitted during the application
/// runtime. It can be transformed into a [axum::response::Response]
/// thanks to [axum::response::IntoResponse].
///
/// It can be created from either an error value that is implementing
/// [Problem] trait or a [sqlx::Error].
#[derive(Debug)]
pub enum HttpError {
    /// An error that is described by an error value implementing
    /// [Problem] trait. It'll be formatted into a HTTP response
    /// following [RFC 9457] recommendations thanks to the
    /// provided methods of [Problem].
    ///
    /// [RFC 9547]: https://datatracker.ietf.org/doc/html/rfc9457
    ProblemError(Box<dyn Problem>),

    /// An error emitted by the SQL backend. It's generally transformed
    /// into a HTTP response representing a server error, that we'll
    /// be formatted into a problem details as defined in [RFC 9457].
    ///
    /// [RFC 9457]: https://datatracker.ietf.org/doc/html/rfc9457
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
