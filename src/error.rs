//! Error and result types for the PII masker.

/// Error types for the PII masker.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to read a file from disk.
    #[error("failed to read {path}: {source}")]
    Read {
        /// Path that could not be read.
        path: String,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse a JSON config file.
    #[error("failed to parse {path}: {source}")]
    Parse {
        /// Path that could not be parsed.
        path: String,
        /// Underlying JSON parse error.
        #[source]
        source: serde_json::Error,
    },

    /// The recognizer catalog is invalid (unknown validator or mask kind).
    #[error("invalid catalog: {0}")]
    Catalog(String),

    /// A recognizer's regex pattern failed to compile.
    #[error("invalid regex for {entity}: {source}")]
    Regex {
        /// Entity whose pattern failed to compile.
        entity: String,
        /// Underlying regex compilation error.
        #[source]
        source: regex::Error,
    },

    /// A generic I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for the PII masker. The error type defaults to [`Error`]
/// but can be overridden for call sites that need a more precise error.
pub type Result<T, E = Error> = std::result::Result<T, E>;
