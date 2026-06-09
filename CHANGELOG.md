# Changelog

All notable changes to this project are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning: [Semantic Versioning](https://semver.org/).

## [0.1.0] — unreleased

### Added

**Engine**
- `Engine` + `EngineBuilder` with `RegexSet` prescreen, per-recognizer `find_iter`,
  validator gate, and greedy overlap resolution.
- Three masking strategies: `Partial` (keep prefix/suffix alnum), `Redact` (labeled
  token `[ENTITY]`), `Hash` (deterministic SHA-256 12-hex token `[ENTITY:aabbccddee11]`).
- `Cow::Borrowed` return on no-PII input (zero allocation).
- `CountrySelection`: `GlobalOnly` (default), `Countries([…])`, `All`.
- Custom recognizer injection, entity-level enable/disable overrides.

**Validators** — all accessible by name from TOML catalog:
`luhn`, `luhn_card`, `iban` (mod-97), `ipv4`, `ipv6`, `verhoeff` (Aadhaar),
`nhs_mod11` (UK NHS), `us_ssn`, `es_nif`, `es_nie`, `it_fiscal`, `pl_pesel`,
`au_abn`, `au_acn`, `au_tfn`, `sg_nric`, `gstin`, `aba_routing`, `fi_hetu`,
`phone_loose`, `de_tax_id`, `fr_nir`, `nl_bsn`, `be_nrn`, `br_cpf`, `br_cnpj`,
`mx_curp`, `no_fnr`, `pt_nif`, `ie_pps`, `ro_cnp`, `tr_tckn`, `cn_ric`,
`jp_mynumber`, `cl_run`, `ar_cuit`, `ru_inn`, `ru_snils`.

**Catalog** (TOML-driven, embedded at compile time):

| Pack | Entities |
|---|---|
| global | CREDIT_CARD, IBAN_CODE, EMAIL_ADDRESS, IP_ADDRESS, IP_ADDRESS_V6, MAC_ADDRESS, CRYPTO_BTC, PHONE_NUMBER, SWIFT_BIC, CRYPTO_ETH |
| US | US_SSN, US_ITIN, US_ABA_ROUTING, US_BANK_NUMBER, US_PASSPORT |
| UK | UK_NHS, UK_NINO, UK_POSTCODE |
| ES | ES_NIF, ES_NIE |
| IT | IT_FISCAL_CODE |
| PL | PL_PESEL |
| SG | SG_NRIC_FIN |
| AU | AU_ABN, AU_ACN, AU_TFN |
| IN | IN_PAN, IN_AADHAAR, IN_GSTIN, IN_VOTER |
| FI | FI_HETU |
| DE | DE_TAX_ID |
| FR | FR_NIR |
| NL | NL_BSN |
| BE | BE_NRN |
| BR | BR_CPF, BR_CNPJ |
| MX | MX_CURP, MX_RFC |
| CA | CA_SIN |
| ZA | ZA_ID |
| SE | SE_PERSONNUMMER |
| NO | NO_FODSELSNUMMER |
| DK | DK_CPR |
| PT | PT_NIF |
| IE | IE_PPS |
| RO | RO_CNP |
| TR | TR_TCKN |
| CN | CN_RIC |
| JP | JP_MYNUMBER |
| CL | CL_RUN |
| AR | AR_CUIT, AR_DNI |
| RU | RU_INN, RU_SNILS |
| ID | ID_NIK |

**CLI** (feature `cli`, default)
- `mask` — stream-mask stdin/file, line-buffered.
- `scan [--json]` — report findings.
- `run [--once]` — daemon beat loop.
- `list [--json] [--all-countries]` — catalog browser.
- Global `--config`, `-v/--verbose`, `RUST_LOG`.

**Daemon** — resumable log-stream masking.
- SHA-256 state token from file birth/modified time + path (log-rotation detection).
- `.linenumber` checkpoint; at-least-once semantics.
- `maxLinesInOneGo` cap; append-mode output.
- Removed vestigial `.running` lock (was always 0 in JS implementation).

**Deferred for future releases:**
- JP `jp_mynumber`: validator set to `none` (algorithm not independently verified
  with ≥3 published samples); recognizer ships with score 0.70.
- Free-text NER (PERSON, LOCATION, ADDRESS, DATE_TIME): out of scope by design —
  no ML/tokenizer dependencies. See `docs/security.md`.

### Removed

- All JavaScript source files (`index.js`, `jobs.js`, `utils.js`).
- `package.json`, `package-lock.json`, `scratch.pad`.
- Old `config/` directory (`master.json`, `jobs.json`, `rules-*.json`).

### Changed

- `config/` replaced by `pii-masker.json` example (new schema: `engine` + `daemon`).
- `AGENTS.md` updated to reflect the Rust architecture.
- `README.md` reduced to install + quickstart; details moved to `docs/`.
