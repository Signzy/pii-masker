//! Masking strategies and the [`apply_mask`] function.

/// Masking strategy for a detected PII entity.
#[derive(Debug, Clone)]
pub enum MaskPolicy {
    /// Replace the entire match with a labeled redaction tag, e.g. `[US_SSN]`.
    Redact {
        /// Label inserted into the redaction template (defaults to entity name).
        label: String,
    },

    /// Keep the first `prefix` and last `suffix` alphanumeric characters visible;
    /// replace the rest with `mask_char`. Non-alnum separators (spaces, dashes, dots)
    /// are preserved verbatim.
    Partial {
        /// Number of leading alphanumeric characters to keep visible.
        prefix: usize,
        /// Number of trailing alphanumeric characters to keep visible.
        suffix: usize,
        /// Character used to replace masked alphanumeric positions.
        mask_char: char,
    },

    /// Replace with a deterministic SHA-256 hash token: `[LABEL:12HEX]`.
    /// The 12 hex chars are the first 12 bytes of the SHA-256 digest of the matched text.
    /// NOTE: deterministic hashing of low-entropy inputs (e.g. 4-digit PINs) is reversible
    /// via rainbow tables. Document this caveat in security docs.
    Hash {
        /// Label inserted into the hash token (defaults to entity name).
        label: String,
    },
}

/// Global options for mask formatting.
#[derive(Debug, Clone)]
pub struct MaskOptions {
    /// Template for Redact strategy: `{entity}` is replaced with the entity name.
    /// Default: `"[{entity}]"`.
    pub redact_template: String,
}

impl Default for MaskOptions {
    fn default() -> Self {
        Self { redact_template: "[{entity}]".to_string() }
    }
}

/// Apply a masking policy to a matched string.
///
/// The `label` is the configured label for the policy (defaults to `entity` if not
/// overridden in the recognizer).
pub fn apply_mask(matched: &str, label: &str, policy: &MaskPolicy, opts: &MaskOptions) -> String {
    match policy {
        MaskPolicy::Redact { label: lbl } => opts.redact_template.replace("{entity}", lbl),
        MaskPolicy::Hash { label: lbl } => {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(matched.as_bytes());
            let digest = hasher.finalize();
            let hex = hex_encode(&digest);
            format!("[{}:{}]", lbl, &hex[..12])
        },
        MaskPolicy::Partial { prefix, suffix, mask_char } => {
            partial_mask(matched, label, *prefix, *suffix, *mask_char, opts)
        },
    }
}

fn partial_mask(
    matched: &str,
    label: &str,
    prefix: usize,
    suffix: usize,
    mask_char: char,
    _opts: &MaskOptions,
) -> String {
    // Count alnum characters and their positions.
    let chars: Vec<char> = matched.chars().collect();
    let alnum_count = chars.iter().filter(|c| c.is_alphanumeric()).count();

    if alnum_count == 0 {
        return _opts.redact_template.replace("{entity}", label);
    }

    let keep_first = prefix.min(alnum_count);
    let keep_last = suffix.min(alnum_count - keep_first);

    let mut seen_alnum = 0usize;
    let mut result = String::with_capacity(matched.len());

    for &ch in &chars {
        if ch.is_alphanumeric() {
            seen_alnum += 1;
            if seen_alnum <= keep_first || seen_alnum > (alnum_count - keep_last) {
                result.push(ch);
            } else {
                result.push(mask_char);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Hex-encode bytes (no `hex` crate — inline).
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
