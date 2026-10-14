use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("path is not on the read-only allowlist: {0}")]
    PathNotAllowed(String),

    #[error("HTTP {status} for {path}: {body}")]
    Http {
        status: u16,
        path: String,
        body: String,
    },

    #[error("resource not supported on this console (404): {0}")]
    Unsupported(String),

    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("json decode failed for {path}: {source}")]
    Decode {
        path: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("classic login failed: HTTP {status}: {body}")]
    Login { status: u16, body: String },

    #[error("{0}")]
    Message(String),
}

pub type Result<T> = std::result::Result<T, Error>;
