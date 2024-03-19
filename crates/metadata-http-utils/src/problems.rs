use http::{header, HeaderMap, HeaderValue, StatusCode};

pub(crate) const CONTENT_TYPE: HeaderValue = HeaderValue::from_static("application/problem+json");

/// `Problem` is a trait to help define the problem details format
/// described in [RFC 9457] to structures that are implementing
/// [std::error::Error] traits.
///
/// It's an opinionated trait as it enforces the trait user to
/// define the `type`, `title` and `detail` fields of the problems
/// details format, even if there is not explicitely tagged as
/// required in the document.
///
/// It's used by the [generic error](metadata_http_utils::HttpError)
/// to transform an error value into an [axum::response::Response].
/// Because it's supposed to be only implemented onto error values,
/// it's mandatory to have [std::error::Error] trait implemented along
/// side [Problem] by either directly implementing it or by using a
/// third-party crate like [thiserror].
///
/// # Example
///
/// ```ignore
/// # use metadata_http_utils::Problem;
/// use thiserror::Error;
///
/// #[derive(Debug, Error)]
/// enum AppError {
///     #[error("Foo is good enough")]
///     Foo,
///     #[error("But bar has something to say: {0}")]
///     Bar(String),
///     #[error("You'll need {egg} eggs, {bread} breads and a lot of spam!")]
///     Spam {
///         egg: i32,
///         bread: i32,
///     },
/// }
///
/// impl Problem for AppError {
///     fn ty(&self) -> String {
///         let ty = match self {
///             Self::Foo => "https://tacocat.dev/errors/foo",
///             Self::Bar(_) => "https://tacocat.dev/errors/bar",
///             Self::Spam { .. } => "https://tacocat.dev/errors/spam",
///         };
///
///         ty.to_owned()
///     }
///
///     fn title(&self) -> String {
///         let title = match self {
///             Self::Foo => "Foo",
///             Self::Bar(_) => "Bar",
///             Self::Spam { .. } => "SPAM",
///         };
///
///         title.to_owned()
///     }
///
///     fn detail(&self) -> String {
///         format!("{self}")
///     }
/// }
/// ```
///
/// [RFC 9457]: https://datatracker.ietf.org/doc/html/rfc9457
/// [thiserror]: https://github.com/dtolnay/thiserror
pub trait Problem: std::error::Error {
    /// A string containing an URI reference that identify the
    /// problem type. It **must** be used by API consumer as
    /// the primary identifier.
    ///
    /// [RFC 9457](https://datatracker.ietf.org/doc/html/rfc9457)
    /// indicates this value should be `"about:blank"` by default.
    fn ty(&self) -> String;

    /// A short, human-readable summary of the problem type.
    fn title(&self) -> String;

    /// A human-readable explanation specific to the occurence
    /// of the problem ought to focus on helping the client
    /// correct the problem, rather than giving debugging
    /// information.
    fn detail(&self) -> String;

    /// A HTTP response status code associated to this occurence
    /// of the problem. If this provided method is returning a
    /// correct HTTP status code, it'll be used to set up the
    /// [axum::response::Response] status code.
    fn status(&self) -> Option<StatusCode> {
        None
    }

    /// A string containing an URI reference that identifies
    /// the specific occurence of the problem in reporting tools.
    /// This field is optional.
    fn instance(&self) -> Option<String> {
        None
    }

    /// A collection of HTTP headers that can are added to a
    /// [axum::response::Response] if the value returned
    /// by the method is not `None`.
    fn headers(&self) -> Option<HeaderMap> {
        None
    }

    fn parts(&self) -> (String, String, String, Option<StatusCode>, Option<String>, Option<HeaderMap>) {
        (
            self.ty(),
            self.title(),
            self.detail(),
            self.status(),
            self.instance(),
            self.headers(),
        )
    }
}

pub(crate) fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(header::CACHE_CONTROL, "no-store".parse().unwrap());
    headers.append(header::CONTENT_TYPE, CONTENT_TYPE);

    headers
}
