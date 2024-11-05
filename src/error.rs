pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown package type: {0}")]
    UnknownPackage(String),
    #[error("{0}")]
    InvalidPurl(#[from] purl::ParseError),
    #[error("{0}")]
    InvalidPurlPackage(#[from] purl::PackageError),
}
