//! Property-based tests using `proptest`.
//!
//! Key invariants:
//! a) `mask()` never panics on arbitrary input.
//! b) Embedding a valid card in random ASCII → the output does not contain the
//!    original card AND `analyze(output)` finds no validated CREDIT_CARD.
//! c) `mask()` output is always valid UTF-8.

use proptest::prelude::*;
use pii_masker::Engine;

fn global() -> Engine {
    Engine::builder().build().expect("engine")
}

proptest! {
    /// mask() must never panic on arbitrary UTF-8 strings.
    #[test]
    fn no_panic_on_arbitrary_string(s in ".*") {
        let e = global();
        let _ = e.mask(&s);
    }

    /// mask() output is always valid UTF-8.
    #[test]
    fn output_is_valid_utf8(s in ".*") {
        let e = global();
        let masked = e.mask(&s);
        prop_assert!(std::str::from_utf8(masked.as_bytes()).is_ok());
    }

    /// A valid Visa card embedded in random ASCII is masked and not leaked.
    #[test]
    fn valid_card_not_leaked(prefix in "[a-z ]{0,20}", suffix in "[a-z ]{0,20}") {
        let card = "4111111111111111";
        let text = format!("{prefix} {card} {suffix}");
        let e = global();
        let masked = e.mask(&text).into_owned();

        // The original card number must not appear in the output
        prop_assert!(
            !masked.contains(card),
            "card leaked in output: {masked}"
        );

        // analyze(output) must find no validated CREDIT_CARD
        let findings = e.analyze(&masked);
        let leaked = findings.iter().any(|f| f.entity == "CREDIT_CARD");
        prop_assert!(!leaked, "re-analysis found CREDIT_CARD in masked output: {masked}");
    }
}
