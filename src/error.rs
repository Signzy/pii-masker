/// Error types for the PII masker.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse {path}: {source}")]
    Parse {
        path: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("invalid catalog: {0}")]
    Catalog(String),

    #[error("invalid regex for {entity}: {source}")]
    Regex {
        entity: String,
        #[source]
        source: regex::Error,
    },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for the PII masker.
pub type Result<T> = std::result::Result<T, Error>;
