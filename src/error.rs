use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("url for key `{0}` is not found")]
    NotFound(String),
    #[error("invalid url syntax")]
    UrlSyntax,
    #[error("DB connection problem")]
    DbFailure,
}
