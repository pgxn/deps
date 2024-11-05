pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown package type: {0}")]
    UnknownPackage(String),
    #[error("{0}")]
    InvalidPurl(#[from] purl::ParseError),
    #[error("{0}")]
    InvalidPurlPackage(#[from] purl::PackageError),
    #[error("Package not found in Repology: {0}")]
    PackageNotFound(String),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    ParseUrl(#[from] url::ParseError),
    #[error("Request failed: {status_code} {message}")]
    FailedRequest { status_code: u16, message: String }
}
