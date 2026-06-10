//! Embedded recognizer catalog loaded from TOML files at compile time.

use std::sync::OnceLock;

use crate::{
    error::{Error, Result},
    recognizer::{RawRecognizer, Recognizer},
};

#[derive(serde::Deserialize)]
struct Wrapper {
    #[serde(default)]
    recognizer: Vec<RawRecognizer>,
}

static RAW: OnceLock<Vec<RawRecognizer>> = OnceLock::new();

fn load_catalog() -> Vec<RawRecognizer> {
    let sources: &[(&str, &str)] = &[
        ("global", include_str!("catalog/global.toml")),
        ("us", include_str!("catalog/us.toml")),
        ("uk", include_str!("catalog/uk.toml")),
        ("es", include_str!("catalog/es.toml")),
        ("it", include_str!("catalog/it.toml")),
        ("pl", include_str!("catalog/pl.toml")),
        ("sg", include_str!("catalog/sg.toml")),
        ("au", include_str!("catalog/au.toml")),
        ("in", include_str!("catalog/in.toml")),
        ("fi", include_str!("catalog/fi.toml")),
        ("de", include_str!("catalog/de.toml")),
        ("fr", include_str!("catalog/fr.toml")),
        ("nl", include_str!("catalog/nl.toml")),
        ("be", include_str!("catalog/be.toml")),
        ("br", include_str!("catalog/br.toml")),
        ("mx", include_str!("catalog/mx.toml")),
        ("ca", include_str!("catalog/ca.toml")),
        ("za", include_str!("catalog/za.toml")),
        ("se", include_str!("catalog/se.toml")),
        ("no", include_str!("catalog/no.toml")),
        ("dk", include_str!("catalog/dk.toml")),
        ("pt", include_str!("catalog/pt.toml")),
        ("ie", include_str!("catalog/ie.toml")),
        ("ro", include_str!("catalog/ro.toml")),
        ("tr", include_str!("catalog/tr.toml")),
        ("cn", include_str!("catalog/cn.toml")),
        ("jp", include_str!("catalog/jp.toml")),
        ("cl", include_str!("catalog/cl.toml")),
        ("ar", include_str!("catalog/ar.toml")),
        ("ru", include_str!("catalog/ru.toml")),
        ("id", include_str!("catalog/id.toml")),
    ];
    let mut all = Vec::new();
    for (name, src) in sources {
        let w: Wrapper =
            toml::from_str(src).unwrap_or_else(|e| panic!("catalog/{name}.toml parse error: {e}"));
        all.extend(w.recognizer);
    }
    all
}

/// Return the full parsed raw catalog (parsed once; subsequent calls are free).
///
/// # Panics
/// Panics if any embedded TOML file fails to parse (compile-time guarantee).
#[must_use]
pub fn raw_catalog() -> &'static [RawRecognizer] {
    RAW.get_or_init(load_catalog)
}

/// Compile the full catalog into a `Vec<Recognizer>`.
///
/// # Errors
/// Returns an error if any recognizer fails to compile (unknown validator or bad regex).
pub fn recognizers() -> Result<Vec<Recognizer>> {
    raw_catalog()
        .iter()
        .map(Recognizer::compile)
        .collect::<Result<Vec<_>>>()
        .map_err(|e| Error::Catalog(e.to_string()))
}
