use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Bad Request ({code}): {message}")]
    BadRequest { code: u64, message: String },
    #[error("Other Error")]
    Other(#[from] reqwest::Error),
    #[error("JSON Error")]
    Json(#[from] serde_json::Error),
    #[error("Unknown Error")]
    Unknown,
}
