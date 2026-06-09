## ADDED Requirements

### Requirement: TOML-driven catalog
Recognizers MUST be defined in TOML files under `src/catalog/` and embedded at compile time via
`include_str!`. The catalog is parsed once into `&'static [RawRecognizer]` via `std::sync::OnceLock`.

#### Scenario: No reparse on repeated calls
- **WHEN** `catalog::raw_catalog()` is called multiple times
- **THEN** parsing happens exactly once; subsequent calls return the same static slice

### Requirement: ASCII-class patterns
Every regex pattern in the catalog MUST use ASCII character classes (`[0-9]`, `[A-Za-z]`,
`[A-Za-z0-9]`). Bare `\d`, `\w`, `\s` MUST NOT appear in any catalog pattern.
This is enforced automatically by `tests/catalog.rs`.

### Requirement: Loose patterns with validator gate
Catalog patterns MUST be deliberately loose (no negative look-ahead, no complex exclusions).
All exclusion logic MUST live in the validator function, not the regex.

### Requirement: Embedded test vectors
Every catalog entry MUST carry:
- `examples`: at minimum 3 valid sample IDs that match the pattern AND pass the validator
- `counter_examples`: at minimum 3 strings that must NOT produce a validated finding

`tests/catalog.rs` MUST automatically iterate every recognizer and assert these vectors.

#### Scenario: Example detection
- **WHEN** a single-recognizer Engine is built for each catalog row
- **THEN** every `examples` entry produces exactly one Finding for that entity

#### Scenario: Counter-example rejection
- **WHEN** a single-recognizer Engine is built for each catalog row
- **THEN** every `counter_examples` entry produces zero findings

### Requirement: Country pack opt-in
Recognizers with a non-empty `country` field MUST be excluded from the Engine unless that country
is explicitly selected. Global recognizers (empty `country`) are always active.

#### Scenario: Default build excludes country packs
- **WHEN** `Engine::builder().build()` is called
- **THEN** IN_PAN, US_SSN, UK_NHS, etc. are not active

#### Scenario: Country selection activates pack
- **WHEN** `Engine::builder().countries(["IN"]).build()` is called
- **THEN** IN_PAN, IN_AADHAAR, IN_GSTIN, IN_VOTER are all active

### Requirement: Source citation
Every catalog entry MUST have a TOML comment citing the authoritative source (Presidio recognizer
page, Wikipedia national ID article, or official government specification).

### Requirement: Core catalog coverage (Phase 2)
The following recognizers MUST be present and passing their test vectors at `0.1.0`:

**Global:** CREDIT_CARD (luhn_card), IBAN_CODE (iban), EMAIL_ADDRESS, IP_ADDRESS v4 (ipv4),
IP_ADDRESS v6 (ipv6), MAC_ADDRESS, CRYPTO_BTC, PHONE_NUMBER (phone_loose)

**US:** US_SSN (us_ssn), US_ITIN, US_ABA_ROUTING (aba_routing), US_BANK_NUMBER, US_PASSPORT

**UK:** UK_NHS (nhs_mod11), UK_NINO, UK_POSTCODE

**ES:** ES_NIF (es_nif), ES_NIE (es_nie)

**IT:** IT_FISCAL_CODE (it_fiscal)

**PL:** PL_PESEL (pl_pesel)

**SG:** SG_NRIC_FIN (sg_nric)

**AU:** AU_ABN (au_abn), AU_ACN (au_acn), AU_TFN (au_tfn)

**IN:** IN_PAN, IN_AADHAAR (verhoeff), IN_GSTIN (gstin), IN_VOTER

**FI:** FI_HETU (fi_hetu)

### Requirement: Phase 4 catalog coverage
The following recognizers MUST be present and passing their test vectors at `0.1.0`:
DE_TAX_ID, FR_NIR, NL_BSN, BE_NRN, BR_CPF, BR_CNPJ, MX_CURP, MX_RFC, CA_SIN, ZA_ID,
SE_PERSONNUMMER, NO_FODSELSNUMMER, DK_CPR, PT_NIF, IE_PPS, RO_CNP, TR_TCKN, CN_RIC, JP_MYNUMBER,
CL_RUN, AR_CUIT, AR_DNI, RU_INN, RU_SNILS, ID_NIK, SWIFT_BIC, CRYPTO_ETH.

If a specific country's format/checksum cannot be verified to the per-entry contract, it MUST be
deferred (noted in CHANGELOG.md) rather than shipped with a guessed regex or wrong validator.
