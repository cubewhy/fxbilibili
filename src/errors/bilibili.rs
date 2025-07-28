use thiserror::Error;

#[derive(Debug, Error)]
pub enum BilibiliError {
    #[error("Failed to send request {0:?}")]
    Http(#[from] reqwest::Error),

    #[error("Bad response: {0}")]
    Api(String),
}
