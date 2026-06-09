//! Raw (deserialized) and compiled recognizer types.

use crate::{
    error::{Error, Result},
    mask::MaskPolicy,
    validator::Validator,
};

/// Raw recognizer as deserialized from a TOML catalog entry.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawRecognizer {
    /// Entity type label, e.g. `"CREDIT_CARD"`.
    pub entity: String,
    /// ISO-3166-1 alpha-2 country code, empty string for global entities.
    #[serde(default)]
    pub country: String,
    /// Regex pattern (ASCII character classes only; no bare `\d`/`\w`).
    pub regex: String,
    /// Validator name (must exist in `Validator::by_name`).
    #[serde(default = "default_validator")]
    pub validator: String,
    /// Confidence score 0.0..=1.0.
    #[serde(default = "default_score")]
    pub score: f32,
    /// Context words that boost detection confidence (currently informational).
    #[serde(default)]
    pub context: Vec<String>,
    /// Whether this recognizer is active when its country pack is selected.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Default masking policy.
    pub mask: RawMask,
    /// Sample IDs that MUST be detected and validated by this recognizer.
    #[serde(default)]
    pub examples: Vec<String>,
    /// Sample strings that MUST NOT produce a validated finding for this recognizer.
    #[serde(default)]
    pub counter_examples: Vec<String>,
}

/// Raw mask configuration as deserialized from TOML.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawMask {
    /// Strategy kind: `"partial"`, `"redact"`, or `"hash"`.
    pub kind: String,
    /// Number of leading alnum chars to keep visible (Partial only).
    #[serde(default)]
    pub prefix: usize,
    /// Number of trailing alnum chars to keep visible (Partial only).
    #[serde(default)]
    pub suffix: usize,
    /// Replacement character for masked alnum positions (Partial only).
    #[serde(default = "default_mask_char")]
    pub mask_char: char,
    /// Override label for the redact/hash token; defaults to entity name.
    #[serde(default)]
    pub label: Option<String>,
}

fn default_validator() -> String {
    "none".to_string()
}
fn default_score() -> f32 {
    0.85
}
fn default_true() -> bool {
    true
}
fn default_mask_char() -> char {
    '*'
}

/// A compiled recognizer ready for use in the engine.
#[derive(Debug)]
pub struct Recognizer {
    /// Entity type label.
    pub entity: String,
    /// Country code, or `None` for global.
    pub country: Option<String>,
    /// Compiled regex.
    pub regex: regex::Regex,
    /// Validator to apply after each regex match.
    pub validator: Validator,
    /// Confidence score.
    pub score: f32,
    /// Context words (informational).
    pub context: Vec<String>,
    /// Whether active when country pack is selected.
    pub enabled: bool,
    /// Resolved masking policy.
    pub mask: MaskPolicy,
}

impl Recognizer {
    /// Compile a [`RawRecognizer`] into a [`Recognizer`].
    ///
    /// # Errors
    /// Returns [`Error::Catalog`] for unknown validator or mask kind, or
    /// [`Error::Regex`] if the pattern fails to compile.
    pub fn compile(raw: &RawRecognizer) -> Result<Self> {
        let validator =
            Validator::by_name(&raw.validator).ok_or_else(|| {
                Error::Catalog(format!(
                    "unknown validator {:?} for entity {}",
                    raw.validator, raw.entity
                ))
            })?;
        let regex = regex::Regex::new(&raw.regex).map_err(|source| Error::Regex {
            entity: raw.entity.clone(),
            source,
        })?;
        let label = raw.mask.label.clone().unwrap_or_else(|| raw.entity.clone());
        let mask = match raw.mask.kind.as_str() {
            "partial" => MaskPolicy::Partial {
                prefix: raw.mask.prefix,
                suffix: raw.mask.suffix,
                mask_char: raw.mask.mask_char,
            },
            "redact" => MaskPolicy::Redact { label },
            "hash" => MaskPolicy::Hash { label },
            other => {
                return Err(Error::Catalog(format!(
                    "unknown mask kind {:?} for entity {}",
                    other, raw.entity
                )))
            }
        };
        Ok(Self {
            entity: raw.entity.clone(),
            country: if raw.country.is_empty() { None } else { Some(raw.country.clone()) },
            regex,
            validator,
            score: raw.score,
            context: raw.context.clone(),
            enabled: raw.enabled,
            mask,
        })
    }
}
