//! A collection of utilities that are used to interact with [axum]
//! framework.
//!
//! It contains an error value envelope that can be converted into
//! a valid HTTP responses and a trait to implement [RFC 9457]
//! problem details specification.
//!
//! [RFC 9457]: https://datatracker.ietf.org/doc/html/rfc9457

mod error;
pub(crate) mod problems;

pub use error::HttpError;
pub use problems::Problem;
