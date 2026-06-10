//! PII detection and masking engine.

use std::borrow::Cow;

use crate::{
    catalog,
    error::Result,
    finding::{resolve_overlaps, Finding},
    mask::{apply_mask, MaskOptions},
    recognizer::{RawRecognizer, Recognizer},
};

/// Controls which country packs are active.
#[derive(Debug, Clone, Default)]
pub enum CountrySelection {
    /// Only global recognizers (no country packs). Default.
    #[default]
    GlobalOnly,
    /// Specific countries, given as ISO-3166-1 alpha-2 codes.
    Countries(Vec<String>),
    /// All registered country packs.
    All,
}

/// The compiled PII detection and masking engine.
pub struct Engine {
    recognizers: Vec<Recognizer>,
    set: regex::RegexSet,
    opts: MaskOptions,
}

impl Engine {
    /// Start building an `Engine`.
    #[must_use]
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }

    /// Analyze `text` and return all non-overlapping validated findings.
    #[must_use]
    pub fn analyze(&self, text: &str) -> Vec<Finding> {
        if text.is_empty() {
            return Vec::new();
        }
        let candidates = self.set.matches(text);
        let mut hits: Vec<Finding> = Vec::new();
        for idx in candidates {
            let rec = &self.recognizers[idx];
            for m in rec.regex.find_iter(text) {
                let span = &text[m.start()..m.end()];
                if rec.validator.validate(span) {
                    hits.push(Finding {
                        entity: rec.entity.clone(),
                        country: rec.country.clone(),
                        start: m.start(),
                        end: m.end(),
                        score: rec.score,
                        recognizer_idx: idx,
                    });
                }
            }
        }
        resolve_overlaps(hits)
    }

    /// Mask all validated PII in `text`, returning `Cow::Borrowed` when nothing changed.
    #[must_use]
    pub fn mask<'a>(&self, text: &'a str) -> Cow<'a, str> {
        self.mask_with(text, &self.opts.clone())
    }

    /// Mask with explicitly provided [`MaskOptions`].
    #[must_use]
    pub fn mask_with<'a>(&self, text: &'a str, opts: &MaskOptions) -> Cow<'a, str> {
        let findings = self.analyze(text);
        if findings.is_empty() {
            return Cow::Borrowed(text);
        }
        let mut out = String::with_capacity(text.len());
        let mut cursor = 0usize;
        for f in &findings {
            if f.start > cursor {
                out.push_str(&text[cursor..f.start]);
            }
            let rec = &self.recognizers[f.recognizer_idx];
            let masked = apply_mask(&text[f.start..f.end], &rec.entity, &rec.mask, opts);
            out.push_str(&masked);
            cursor = f.end;
        }
        if cursor < text.len() {
            out.push_str(&text[cursor..]);
        }
        Cow::Owned(out)
    }

    /// Access the active recognizers (for `list` subcommand).
    #[must_use]
    pub fn recognizers(&self) -> &[Recognizer] {
        &self.recognizers
    }
}

/// Builder for [`Engine`].
#[derive(Default)]
pub struct EngineBuilder {
    country_selection: CountrySelection,
    disable_entities: Vec<String>,
    enable_entities: Vec<String>,
    custom_recognizers: Vec<RawRecognizer>,
    opts: MaskOptions,
}

impl EngineBuilder {
    /// Activate specific country packs (ISO-3166-1 alpha-2, e.g. `["US", "IN"]`).
    #[must_use]
    pub fn countries(mut self, codes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.country_selection = CountrySelection::Countries(
            codes.into_iter().map(|s| s.into().to_ascii_uppercase()).collect(),
        );
        self
    }

    /// Activate all country packs.
    #[must_use]
    pub fn all_countries(mut self) -> Self {
        self.country_selection = CountrySelection::All;
        self
    }

    /// Disable specific entity types even if their country pack is active.
    #[must_use]
    pub fn disable_entities(
        mut self,
        entities: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.disable_entities.extend(entities.into_iter().map(Into::into));
        self
    }

    /// Force-enable specific entity types even if they would otherwise be excluded.
    #[must_use]
    pub fn enable_entities(
        mut self,
        entities: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.enable_entities.extend(entities.into_iter().map(Into::into));
        self
    }

    /// Add a custom recognizer from raw config.
    #[must_use]
    pub fn add_recognizer(mut self, raw: RawRecognizer) -> Self {
        self.custom_recognizers.push(raw);
        self
    }

    /// Override global masking options.
    #[must_use]
    pub fn mask_options(mut self, opts: MaskOptions) -> Self {
        self.opts = opts;
        self
    }

    /// Build the [`Engine`].
    ///
    /// # Errors
    /// Returns an error if catalog loading or regex compilation fails.
    pub fn build(self) -> Result<Engine> {
        let mut catalog_recs = catalog::recognizers()?;

        // Filter by country selection and enabled flags
        catalog_recs.retain(|r| {
            if !r.enabled {
                return false;
            }
            if self.disable_entities.contains(&r.entity) {
                return false;
            }
            if self.enable_entities.contains(&r.entity) {
                return true;
            }
            match &self.country_selection {
                CountrySelection::GlobalOnly => r.country.is_none(),
                CountrySelection::All => true,
                CountrySelection::Countries(codes) => {
                    r.country.is_none()
                        || r.country
                            .as_deref()
                            .map(|c| codes.contains(&c.to_ascii_uppercase()))
                            .unwrap_or(false)
                },
            }
        });

        // Append custom recognizers
        for raw in &self.custom_recognizers {
            catalog_recs.push(Recognizer::compile(raw)?);
        }

        let patterns: Vec<String> =
            catalog_recs.iter().map(|r| r.regex.as_str().to_string()).collect();
        let set = regex::RegexSet::new(&patterns).map_err(|source| crate::error::Error::Regex {
            entity: "RegexSet".to_string(),
            source,
        })?;

        Ok(Engine { recognizers: catalog_recs, set, opts: self.opts })
    }
}
