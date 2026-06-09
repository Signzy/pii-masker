//! Banking-grade PII detection and masking.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use pii_masker::Engine;
//!
//! fn main() -> pii_masker::Result<()> {
//!     let engine = Engine::builder()
//!         .countries(["US", "IN"])
//!         .build()?;
//!
//!     let masked = engine.mask("card 4111 1111 1111 1111, ssn 219-09-9999");
//!     println!("{masked}");
//!     Ok(())
//! }
//! ```

pub mod catalog;
pub mod config;
pub mod daemon;
pub mod engine;
pub mod error;
pub mod finding;
pub mod mask;
pub mod recognizer;
pub mod validator;

#[cfg(feature = "cli")]
pub mod cli;

// Re-exports
pub use engine::{Engine, EngineBuilder};
pub use error::{Error, Result};
pub use finding::Finding;
pub use mask::{MaskOptions, MaskPolicy};
