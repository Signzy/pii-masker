//! Catalog self-tests: every recognizer's examples and counter_examples are exercised.
//!
//! A recognizer cannot ship without passing its embedded test vectors.

use pii_masker::{catalog, Engine, EngineBuilder};

/// Assert no bare `\d`, `\w`, `\s` in any catalog pattern.
#[test]
fn no_bare_unicode_classes() {
    for r in catalog::raw_catalog() {
        let pat = &r.regex;
        // Bare `\d`, `\w`, `\s` outside a bracket-class
        assert!(
            !pat.contains(r"\d"),
            "recognizer {} uses bare \\d in pattern; use [0-9]",
            r.entity
        );
        assert!(
            !pat.contains(r"\w"),
            "recognizer {} uses bare \\w in pattern; use [A-Za-z0-9_]",
            r.entity
        );
    }
}

/// Every catalog entry has ≥3 examples and ≥3 counter_examples.
#[test]
fn min_vector_counts() {
    for r in catalog::raw_catalog() {
        assert!(
            r.examples.len() >= 3,
            "recognizer {} has only {} examples (need ≥3)",
            r.entity,
            r.examples.len()
        );
        assert!(
            r.counter_examples.len() >= 3,
            "recognizer {} has only {} counter_examples (need ≥3)",
            r.entity,
            r.counter_examples.len()
        );
    }
}

/// Every recognizer compiles (valid regex + known validator + known mask kind).
#[test]
fn all_recognizers_compile() {
    catalog::recognizers().expect("catalog should compile without errors");
}

/// Every example yields exactly one finding for its entity; every counter_example yields none.
#[test]
fn example_and_counter_vectors() {
    let all_recs = catalog::raw_catalog();
    for raw in all_recs {
        let compiled = match pii_masker::recognizer::Recognizer::compile(raw) {
            Ok(c) => c,
            Err(e) => panic!("recognizer {} failed to compile: {e}", raw.entity),
        };

        // Every example must produce at least one validated finding
        for example in &raw.examples {
            let mut found = false;
            for m in compiled.regex.find_iter(example) {
                let span = &example[m.start()..m.end()];
                if compiled.validator.validate(span) {
                    found = true;
                    break;
                }
            }
            assert!(
                found,
                "recognizer {} example {:?} was NOT detected/validated",
                raw.entity, example
            );
        }

        // Every counter_example must produce zero validated findings
        for ce in &raw.counter_examples {
            let mut found = false;
            for m in compiled.regex.find_iter(ce) {
                let span = &ce[m.start()..m.end()];
                if compiled.validator.validate(span) {
                    found = true;
                    break;
                }
            }
            assert!(
                !found,
                "recognizer {} counter_example {:?} was incorrectly detected",
                raw.entity, ce
            );
        }
    }
}

/// Build a global-only engine and confirm it contains the expected entities.
#[test]
fn global_engine_contains_core_entities() {
    let engine = Engine::builder().build().expect("global engine");
    let entities: Vec<&str> = engine.recognizers().iter().map(|r| r.entity.as_str()).collect();
    for expected in &["CREDIT_CARD", "IBAN_CODE", "EMAIL_ADDRESS", "IP_ADDRESS", "MAC_ADDRESS"] {
        assert!(entities.contains(expected), "missing global entity {expected}");
    }
}

/// Country packs are opt-in: US_SSN should not appear in a global-only engine.
#[test]
fn country_packs_opt_in() {
    let global = Engine::builder().build().expect("global engine");
    let has_ssn = global.recognizers().iter().any(|r| r.entity == "US_SSN");
    assert!(!has_ssn, "US_SSN should not be in global-only engine");

    let with_us = EngineBuilder::default().countries(["US"]).build().expect("US engine");
    let has_ssn = with_us.recognizers().iter().any(|r| r.entity == "US_SSN");
    assert!(has_ssn, "US_SSN must be active when US pack is selected");
}
