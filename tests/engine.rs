//! Integration tests for the detection engine.

use std::borrow::Cow;

use pii_masker::{Engine, EngineBuilder, MaskOptions, MaskPolicy};

fn global_engine() -> Engine {
    Engine::builder().build().expect("global engine")
}

fn us_engine() -> Engine {
    EngineBuilder::default().countries(["US"]).build().expect("US engine")
}

// ── Basic detection ──────────────────────────────────────────────────────────

#[test]
fn detects_valid_credit_card() {
    let e = global_engine();
    let findings = e.analyze("pay with card 4111111111111111 now");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].entity, "CREDIT_CARD");
}

#[test]
fn invalid_luhn_not_detected() {
    let e = global_engine();
    let findings = e.analyze("bad card 4111111111111112 here");
    let cards: Vec<_> = findings.iter().filter(|f| f.entity == "CREDIT_CARD").collect();
    assert!(cards.is_empty(), "Luhn-invalid card must not be detected");
}

#[test]
fn detects_ipv4() {
    let e = global_engine();
    let findings = e.analyze("remote 192.168.1.1 connected");
    assert!(findings.iter().any(|f| f.entity == "IP_ADDRESS"));
}

#[test]
fn detects_email() {
    let e = global_engine();
    let findings = e.analyze("contact user@example.com for info");
    assert!(findings.iter().any(|f| f.entity == "EMAIL_ADDRESS"));
}

// ── Multi-entity line ────────────────────────────────────────────────────────

#[test]
fn multi_entity_line() {
    let e = global_engine();
    let text = "card 4111111111111111 ip 10.0.0.1 email test@example.com";
    let findings = e.analyze(text);
    let entities: Vec<&str> = findings.iter().map(|f| f.entity.as_str()).collect();
    assert!(entities.contains(&"CREDIT_CARD"), "should find card");
    assert!(entities.contains(&"IP_ADDRESS"), "should find ip");
    assert!(entities.contains(&"EMAIL_ADDRESS"), "should find email");
}

// ── Overlap resolution ───────────────────────────────────────────────────────

#[test]
fn credit_card_wins_over_phone_overlap() {
    // 16 digits can look like a phone pattern at low score
    let e = global_engine();
    let findings = e.analyze("num 4111111111111111 end");
    let cc = findings.iter().any(|f| f.entity == "CREDIT_CARD");
    assert!(cc, "CREDIT_CARD should be detected in 16-digit overlap");
    // overlapping findings should be collapsed
    for i in 0..findings.len() {
        for j in (i + 1)..findings.len() {
            assert!(
                findings[i].end <= findings[j].start,
                "overlapping findings detected: {:?} and {:?}",
                findings[i],
                findings[j]
            );
        }
    }
}

// ── Masking strategies ───────────────────────────────────────────────────────

#[test]
fn mask_partial_card() {
    let e = global_engine();
    let masked = e.mask("card 4111 1111 1111 1111 end");
    // Last 4 alnum visible
    assert!(masked.contains("1111"), "last 4 digits should be visible");
    assert!(!masked.contains("4111 1111 1111 1111"), "full card should be masked");
}

#[test]
fn mask_redact_ip() {
    let e = global_engine();
    let masked = e.mask("ip 10.0.0.1 used");
    assert!(masked.contains("[IP_ADDRESS]"), "IP should be redacted");
    assert!(!masked.contains("10.0.0.1"), "original IP should not appear");
}

#[test]
fn mask_hash_strategy() {
    // Direct strategy test via apply_mask
    use pii_masker::mask::apply_mask;
    let policy = MaskPolicy::Hash { label: "TEST".to_string() };
    let result = apply_mask("4111111111111111", "TEST", &policy, &MaskOptions::default());
    assert!(result.starts_with("[TEST:"));
    assert!(result.ends_with(']'));
    let inner = &result[6..result.len() - 1];
    assert_eq!(inner.len(), 12, "hash token should be 12 hex chars");
    assert!(inner.chars().all(|c| c.is_ascii_hexdigit()), "must be hex");
}

#[test]
fn hash_is_deterministic() {
    use pii_masker::mask::{apply_mask, MaskOptions};
    let policy = MaskPolicy::Hash { label: "X".to_string() };
    let opts = MaskOptions::default();
    let r1 = apply_mask("hello", "X", &policy, &opts);
    let r2 = apply_mask("hello", "X", &policy, &opts);
    assert_eq!(r1, r2);
}

// ── Cow::Borrowed on no-PII ──────────────────────────────────────────────────

#[test]
fn no_pii_returns_borrowed() {
    let e = global_engine();
    let text = "nothing here to mask";
    let result = e.mask(text);
    assert!(matches!(result, Cow::Borrowed(_)), "clean text must return Cow::Borrowed");
}

// ── Idempotence ──────────────────────────────────────────────────────────────

#[test]
fn mask_is_idempotent() {
    let e = global_engine();
    let text = "pay 4111111111111111 from 10.0.0.1";
    let once = e.mask(text).into_owned();
    let twice = e.mask(&once).into_owned();
    assert_eq!(once, twice, "masking twice must equal masking once");
}

// ── Empty / edge inputs ──────────────────────────────────────────────────────

#[test]
fn empty_input_safe() {
    let e = global_engine();
    assert!(e.analyze("").is_empty());
    let r = e.mask("");
    assert!(r.is_empty());
}

#[test]
fn unicode_line_safe() {
    let e = global_engine();
    // Multi-byte chars should not cause panics or corrupt output
    let text = "こんにちは card 4111111111111111 世界";
    let result = e.mask(text);
    assert!(std::str::from_utf8(result.as_bytes()).is_ok(), "must be valid UTF-8");
    assert!(!result.contains("4111111111111111"), "card must be masked");
}

// ── Country selection ────────────────────────────────────────────────────────

#[test]
fn us_ssn_detected_when_us_selected() {
    let e = us_engine();
    let findings = e.analyze("ssn 219-09-9999 here");
    assert!(findings.iter().any(|f| f.entity == "US_SSN"));
}

#[test]
fn us_ssn_absent_in_global() {
    let e = global_engine();
    let findings = e.analyze("ssn 219-09-9999 here");
    assert!(!findings.iter().any(|f| f.entity == "US_SSN"));
}

#[test]
fn disable_entity() {
    let e = EngineBuilder::default().disable_entities(["CREDIT_CARD"]).build().expect("engine");
    let findings = e.analyze("card 4111111111111111 here");
    assert!(!findings.iter().any(|f| f.entity == "CREDIT_CARD"));
}

// ── MaskOptions custom template ──────────────────────────────────────────────

#[test]
fn custom_redact_template() {
    let e = global_engine();
    let opts = MaskOptions { redact_template: "<REDACTED:{entity}>".to_string() };
    let result = e.mask_with("ip 10.0.0.1 here", &opts);
    assert!(result.contains("<REDACTED:IP_ADDRESS>"), "custom template not applied");
}
