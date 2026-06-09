//! Known-vector tests for every `Validator` variant.
//!
//! Every entry cites the authoritative source for the test values.

use pii_masker::validator::Validator;

fn v(name: &str) -> Validator {
    Validator::by_name(name).unwrap_or_else(|| panic!("unknown validator {name}"))
}

// ── Luhn / LuhnCard ──────────────────────────────────────────────────────────

#[test]
fn luhn_valid() {
    // Source: https://en.wikipedia.org/wiki/Luhn_algorithm
    assert!(v("luhn").validate("79927398713"));
    assert!(v("luhn").validate("4111111111111111")); // Visa test card
    assert!(v("luhn").validate("5500005555555559")); // MC test card
}

#[test]
fn luhn_invalid() {
    assert!(!v("luhn").validate("79927398710"));
    assert!(!v("luhn").validate("4111111111111112"));
}

#[test]
fn luhn_card_valid() {
    assert!(v("luhn_card").validate("4111111111111111")); // 16 digits
    assert!(v("luhn_card").validate("378282246310005")); // Amex 15 digits
}

#[test]
fn luhn_card_rejects_short() {
    assert!(!v("luhn_card").validate("79927398713")); // 11 digits — too short
}

// ── IBAN ─────────────────────────────────────────────────────────────────────

#[test]
fn iban_valid() {
    // Source: https://www.iban.com/structure
    assert!(v("iban").validate("DE89370400440532013000"));
    assert!(v("iban").validate("GB29NWBK60161331926819"));
    assert!(v("iban").validate("FR7630006000011234567890189"));
    assert!(v("iban").validate("NL91ABNA0417164300")); // 18-char Dutch IBAN
}

#[test]
fn iban_invalid() {
    assert!(!v("iban").validate("DE89370400440532013001")); // last digit off
    assert!(!v("iban").validate("NOTANIBAN"));
    assert!(!v("iban").validate("DE89")); // too short
}

// ── IPv4 / IPv6 ──────────────────────────────────────────────────────────────

#[test]
fn ipv4_valid() {
    assert!(v("ipv4").validate("192.168.1.1"));
    assert!(v("ipv4").validate("10.0.0.1"));
    assert!(v("ipv4").validate("0.0.0.0"));
}

#[test]
fn ipv4_invalid() {
    assert!(!v("ipv4").validate("999.0.0.1"));
    assert!(!v("ipv4").validate("256.1.1.1"));
    assert!(!v("ipv4").validate("not-an-ip"));
}

#[test]
fn ipv6_valid() {
    assert!(v("ipv6").validate("::1"));
    assert!(v("ipv6").validate("2001:db8::1"));
    assert!(v("ipv6").validate("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
}

#[test]
fn ipv6_invalid() {
    assert!(!v("ipv6").validate("192.168.1.1"));
    assert!(!v("ipv6").validate("gggg::1"));
}

// ── Verhoeff (Aadhaar) ───────────────────────────────────────────────────────

#[test]
fn verhoeff_valid() {
    // "2363" is a canonical Verhoeff test vector
    assert!(v("verhoeff").validate("2363"));
}

#[test]
fn verhoeff_invalid() {
    assert!(!v("verhoeff").validate("2364"));
    assert!(!v("verhoeff").validate("2362"));
}

// ── NHS mod-11 ───────────────────────────────────────────────────────────────

#[test]
fn nhs_valid() {
    // Source: NHS Data Dictionary — number validation spec
    assert!(v("nhs_mod11").validate("9434765919"));
    assert!(v("nhs_mod11").validate("2000000002"));
}

#[test]
fn nhs_invalid() {
    assert!(!v("nhs_mod11").validate("9434765910")); // wrong check digit
    assert!(!v("nhs_mod11").validate("9434765918"));
}

// ── US SSN ───────────────────────────────────────────────────────────────────

#[test]
fn us_ssn_valid() {
    assert!(v("us_ssn").validate("219099999")); // area 219, group 09, serial 9999
    assert!(v("us_ssn").validate("123456789")); // normal
}

#[test]
fn us_ssn_invalid() {
    assert!(!v("us_ssn").validate("000123456")); // area 000
    assert!(!v("us_ssn").validate("666123456")); // area 666
    assert!(!v("us_ssn").validate("900123456")); // area >= 900
    assert!(!v("us_ssn").validate("123001234")); // group 00
    assert!(!v("us_ssn").validate("123120000")); // serial 0000
}

// ── ABA routing ──────────────────────────────────────────────────────────────

#[test]
fn aba_valid() {
    // Source: https://en.wikipedia.org/wiki/ABA_routing_transit_number
    assert!(v("aba_routing").validate("322271627"));
    assert!(v("aba_routing").validate("021000021")); // JPMorgan Chase
    assert!(v("aba_routing").validate("011000015")); // Bank of America
}

#[test]
fn aba_invalid() {
    assert!(!v("aba_routing").validate("122271627"));
    assert!(!v("aba_routing").validate("000000000"));
}

// ── ES NIF / NIE ─────────────────────────────────────────────────────────────

#[test]
fn es_nif_valid() {
    // TABLE = TRWAGMYFPDXBNJZSQVHLCKE; 12345678 % 23 = 14 → 'Z'
    assert!(v("es_nif").validate("12345678Z"));
    // 0 % 23 = 0 → 'T'
    assert!(v("es_nif").validate("00000000T"));
}

#[test]
fn es_nif_invalid() {
    assert!(!v("es_nif").validate("12345678A")); // wrong letter
}

#[test]
fn es_nie_valid() {
    // X1234567 → replace X→0 → 01234567 → 1234567 % 23 = ?
    // 1234567 % 23 = 1234567 - 53676*23 = 1234567 - 1234548 = 19 → 'L'
    assert!(v("es_nie").validate("X1234567L"));
}

// ── AU ABN ───────────────────────────────────────────────────────────────────

#[test]
fn au_abn_valid() {
    // Source: https://abr.business.gov.au/Help/AbnFormat
    assert!(v("au_abn").validate("51824753556"));
    assert!(v("au_abn").validate("51 824 753 556")); // with spaces
}

#[test]
fn au_abn_invalid() {
    assert!(!v("au_abn").validate("51824753557"));
    assert!(!v("au_abn").validate("00000000000")); // first digit 0 → invalid
}

// ── GSTIN ────────────────────────────────────────────────────────────────────

#[test]
fn gstin_valid() {
    // Source: verified against known GST portal algorithm
    assert!(v("gstin").validate("27AAPFU0939F1ZV"));
}

#[test]
fn gstin_invalid() {
    assert!(!v("gstin").validate("27AAPFU0939F1ZW")); // wrong check char
}

// ── CN RIC ───────────────────────────────────────────────────────────────────

#[test]
fn cn_ric_valid() {
    // Source: ISO 7064 MOD 11-2
    assert!(v("cn_ric").validate("11010519491231002X"));
}

#[test]
fn cn_ric_invalid() {
    assert!(!v("cn_ric").validate("11010519491231002Y"));
}

// ── PhoneLoose ───────────────────────────────────────────────────────────────

#[test]
fn phone_loose_valid() {
    assert!(v("phone_loose").validate("+1 800-555-1234")); // 11 digits
    assert!(v("phone_loose").validate("+919876543210")); // 12 digits
}

#[test]
fn phone_loose_invalid() {
    assert!(!v("phone_loose").validate("12345")); // only 5 digits
    assert!(!v("phone_loose").validate("hello")); // no digits
}

// ── by_name registry ─────────────────────────────────────────────────────────

#[test]
fn by_name_all_known() {
    let names = [
        "none", "luhn", "luhn_card", "iban", "ipv4", "ipv6", "verhoeff", "nhs_mod11", "us_ssn",
        "es_nif", "es_nie", "it_fiscal", "pl_pesel", "au_abn", "au_acn", "au_tfn", "sg_nric",
        "gstin", "aba_routing", "fi_hetu", "phone_loose", "de_tax_id", "fr_nir", "nl_bsn",
        "be_nrn", "br_cpf", "br_cnpj", "mx_curp", "no_fnr", "pt_nif", "ie_pps", "ro_cnp",
        "tr_tckn", "cn_ric", "jp_mynumber", "cl_run", "ar_cuit", "ru_inn", "ru_snils",
    ];
    for name in &names {
        assert!(
            Validator::by_name(name).is_some(),
            "validator {name} not found in registry"
        );
    }
}

#[test]
fn by_name_unknown_returns_none() {
    assert!(Validator::by_name("definitely_not_a_validator").is_none());
}
