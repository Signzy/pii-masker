//! Configuration loading for the PII masker engine and daemon.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{
    error::{Error, Result},
    recognizer::{RawMask, RawRecognizer},
};

/// Top-level configuration (`pii-masker.json`).
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Config {
    /// Engine detection and masking settings.
    pub engine: EngineConfig,
    /// Daemon job and scheduling settings.
    pub daemon: DaemonConfig,
}

/// Deserializes from either the string `"all"` or a `Vec<String>`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(untagged)]
pub enum CountrySel {
    /// Global recognizers only (default).
    #[default]
    None,
    /// All country packs.
    All(AllMarker),
    /// Specific ISO-3166-1 alpha-2 country codes.
    List(Vec<String>),
}

/// Marker type that only deserializes from the string `"all"`.
#[derive(Debug, Clone)]
pub struct AllMarker;

impl<'de> Deserialize<'de> for AllMarker {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        if s == "all" {
            Ok(AllMarker)
        } else {
            Err(serde::de::Error::custom("expected \"all\""))
        }
    }
}

/// Engine-side configuration.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EngineConfig {
    /// Country pack selection.
    pub countries: CountrySel,
    /// Entity types to disable even when their country pack is active.
    pub disable_entities: Vec<String>,
    /// Entity types to force-enable.
    pub enable_entities: Vec<String>,
    /// Per-entity mask overrides keyed by entity name.
    pub mask_overrides: HashMap<String, RawMask>,
    /// Additional custom recognizers injected into the engine.
    pub custom_recognizers: Vec<RawRecognizer>,
}

impl EngineConfig {
    /// Build an [`Engine`][crate::engine::Engine] from this configuration.
    ///
    /// # Errors
    /// Propagates catalog or regex-compilation errors.
    pub fn build_engine(&self) -> Result<crate::engine::Engine> {
        use crate::engine::EngineBuilder;
        let mut b = EngineBuilder::default();
        b = match &self.countries {
            CountrySel::None => b,
            CountrySel::All(_) => b.all_countries(),
            CountrySel::List(codes) => b.countries(codes.iter().cloned()),
        };
        if !self.disable_entities.is_empty() {
            b = b.disable_entities(self.disable_entities.iter().cloned());
        }
        if !self.enable_entities.is_empty() {
            b = b.enable_entities(self.enable_entities.iter().cloned());
        }
        for raw in &self.custom_recognizers {
            b = b.add_recognizer(raw.clone());
        }
        b.build()
    }
}

/// A single daemon job definition.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Job {
    /// Identifier used in log messages.
    pub job_name: String,
    /// Input and output file paths.
    pub files: JobFiles,
}

/// File paths for a daemon job.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct JobFiles {
    /// Path to the log file to read.
    pub input: PathBuf,
    /// Path to the masked output file.
    pub output: PathBuf,
}

/// Daemon scheduling and job configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DaemonConfig {
    /// Milliseconds to sleep between beats.
    pub beat_interval: u64,
    /// Directory for checkpoint (`.linenumber`) files.
    pub tmp_directory: PathBuf,
    /// Maximum lines to process per job per beat.
    pub max_lines_in_one_go: usize,
    /// Logging verbosity (`debug`, `verbose`, `error`, `noLog`).
    pub run_mode: Option<String>,
    /// Configured jobs.
    pub jobs: Vec<Job>,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            beat_interval: 5000,
            tmp_directory: PathBuf::from("/tmp"),
            max_lines_in_one_go: 5000,
            run_mode: None,
            jobs: Vec::new(),
        }
    }
}

impl DaemonConfig {
    /// Map `runMode` to an `env_logger` filter string.
    #[must_use]
    pub fn log_filter(&self) -> &'static str {
        match self.run_mode.as_deref() {
            Some("debug") | Some("verbose") => "debug",
            Some("error") => "error",
            Some("noLog") => "off",
            _ => "info",
        }
    }
}

/// Load a [`Config`] from a JSON file path.
///
/// # Errors
/// Returns [`Error::Read`] on I/O failure or [`Error::Parse`] on JSON parse failure.
pub fn load(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&content).map_err(|source| Error::Parse {
        path: path.display().to_string(),
        source,
    })
}

/// Load config from `path`, falling back to `pii-masker.json`, then built-in defaults.
#[must_use]
pub fn load_or_default(path: Option<&Path>) -> Config {
    if let Some(p) = path {
        if let Ok(c) = load(p) {
            return c;
        }
    }
    let default_path = Path::new("pii-masker.json");
    if default_path.exists() {
        if let Ok(c) = load(default_path) {
            return c;
        }
    }
    Config::default()
}
