//! CLI integration tests using `assert_cmd`.

use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;

fn pii_masker() -> Command {
    Command::cargo_bin("pii-masker").unwrap()
}

// ── mask subcommand ───────────────────────────────────────────────────────────

#[test]
fn mask_stdin_card() {
    pii_masker()
        .args(["mask"])
        .write_stdin("pay with 4111111111111111 now\n")
        .assert()
        .success()
        .stdout(predicates::str::contains("1111"))    // last 4 visible
        .stdout(predicates::str::contains("4111111111111111").not()); // full card gone
}

#[test]
fn mask_invalid_luhn_passes_through() {
    let bad_card = "4111111111111112";
    pii_masker()
        .args(["mask"])
        .write_stdin(format!("card {bad_card}\n"))
        .assert()
        .success()
        .stdout(predicates::str::contains(bad_card)); // unchanged
}

#[test]
fn mask_ip_redacted() {
    pii_masker()
        .args(["mask"])
        .write_stdin("ip 10.0.0.1 here\n")
        .assert()
        .success()
        .stdout(predicates::str::contains("[IP_ADDRESS]"))
        .stdout(predicates::str::contains("10.0.0.1").not());
}

#[test]
fn mask_with_country_flag() {
    // SSN should only be masked when --country US is passed
    pii_masker()
        .args(["mask", "--country", "US"])
        .write_stdin("ssn 219-09-9999 here\n")
        .assert()
        .success()
        .stdout(predicates::str::contains("[US_SSN]"));
}

#[test]
fn mask_ssn_absent_without_country() {
    pii_masker()
        .args(["mask"])
        .write_stdin("ssn 219-09-9999 here\n")
        .assert()
        .success()
        .stdout(predicates::str::contains("219-09-9999")); // not masked
}

// ── scan subcommand ───────────────────────────────────────────────────────────

#[test]
fn scan_json_finds_credit_card() {
    let output = pii_masker()
        .args(["scan", "--json"])
        .write_stdin("4111111111111111\n")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).unwrap();
    let arr: serde_json::Value = serde_json::from_str(&s).expect("scan --json must produce valid JSON");
    let entities: Vec<&str> = arr
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v["entity"].as_str())
        .collect();
    assert!(entities.contains(&"CREDIT_CARD"), "must find CREDIT_CARD: {s}");
}

#[test]
fn scan_no_findings_exits_zero() {
    pii_masker()
        .args(["scan", "--json"])
        .write_stdin("nothing to see here\n")
        .assert()
        .success()
        .stdout("[\n]\n"); // empty JSON array
}

// ── list subcommand ───────────────────────────────────────────────────────────

#[test]
fn list_shows_credit_card() {
    pii_masker()
        .args(["list"])
        .assert()
        .success()
        .stdout(predicates::str::contains("CREDIT_CARD"));
}

#[test]
fn list_json_is_array() {
    let output = pii_masker()
        .args(["list", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let s = String::from_utf8(output).unwrap();
    let arr: serde_json::Value = serde_json::from_str(&s).expect("list --json must be valid JSON");
    assert!(arr.is_array());
    assert!(!arr.as_array().unwrap().is_empty());
}

#[test]
fn list_all_countries_includes_us_ssn() {
    pii_masker()
        .args(["list", "--all-countries"])
        .assert()
        .success()
        .stdout(predicates::str::contains("US_SSN"));
}

// ── help / error ──────────────────────────────────────────────────────────────

#[test]
fn no_args_prints_help() {
    // clap returns exit code 2 for missing subcommand
    pii_masker().assert().failure();
}
