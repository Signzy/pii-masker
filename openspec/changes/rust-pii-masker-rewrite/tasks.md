## 1. Scaffold & Manifest

- [x] 1.1 Create `Cargo.toml` with lib + bin targets, features (cli, phone), all deps at pinned versions, lints table, bench entry
- [x] 1.2 Update `.gitignore` to Rust-only (remove all Node.js content)
- [x] 1.3 Update `.editorconfig` with `*.md` and `*.{json,toml,yml,yaml}` indent-size overrides
- [x] 1.4 Confirm `rustfmt.toml` settings (edition 2021, max_width 100)
- [x] 1.5 Create `.github/workflows/ci.yml` (fmt check, clippy -D warnings, test --all-features, build --release)
- [x] 1.6 Create `.github/workflows/release.yml` (tag v* → cargo publish --dry-run then publish)

## 2. Engine Core — Error & Types

- [x] 2.1 Implement `src/error.rs`: `Error` enum (Read, Parse, Catalog, Regex, Io) + `Result<T>` alias using `thiserror`
- [x] 2.2 Implement `src/mask.rs`: `MaskPolicy` enum (Redact, Partial, Hash), `MaskOptions` with default, `apply_mask()` using `sha2` for Hash strategy
- [x] 2.3 Implement `src/finding.rs`: `Finding` struct (entity, country, start, end, score, recognizer_idx) + `resolve_overlaps()` greedy algorithm

## 3. Validator Registry

- [x] 3.1 Implement `src/validator.rs` skeleton: `Validator` enum + `by_name()` registry + `validate()` dispatch + `digits()` helper
- [x] 3.2 Add Luhn and LuhnCard algorithms with unit tests (4111111111111111 valid, 4111111111111112 invalid)
- [x] 3.3 Add IBAN mod-97 with unit tests (DE89370400440532013000 valid, GB29NWBK60161331926819 valid)
- [x] 3.4 Add Verhoeff (Aadhaar) with D/P/INV tables and unit tests (≥3 published valid samples)
- [x] 3.5 Add NhsMod11 with unit tests (9434765919 valid)
- [x] 3.6 Add UsSsn structural validator with unit tests (219-09-9999 valid, 000-09-9999 invalid)
- [x] 3.7 Add AbaRouting with unit tests (322271627 valid)
- [x] 3.8 Add EsNif / EsNie with unit tests
- [x] 3.9 Add ItFiscal with odd/even position tables and unit tests (≥3 published valid samples)
- [x] 3.10 Add PlPesel with unit tests (≥3 published valid samples)
- [x] 3.11 Add AuAbn / AuAcn / AuTfn with unit tests (51824753556 valid ABN)
- [x] 3.12 Add SgNric with prefix-keyed tables and unit tests (≥3 published valid samples)
- [x] 3.13 Add Gstin with base-36 mod-36 and unit tests (≥3 published valid samples)
- [x] 3.14 Add FiHetu with control character table and unit tests (≥3 published valid samples)
- [x] 3.15 Add Ipv4 / Ipv6 (std::net parse delegation) and PhoneLoose (digit count 7..=15)

## 4. Recognizer & Catalog Loader

- [x] 4.1 Implement `src/recognizer.rs`: `RawRecognizer` (serde Deserialize), `RawMask`, compiled `Recognizer`, `Recognizer::compile()`
- [x] 4.2 Implement `src/catalog.rs`: TOML wrapper struct, `include_str!` list for all country files, `raw_catalog()` via OnceLock, `recognizers()` compiling all
- [x] 4.3 Create `src/catalog/global.toml`: CREDIT_CARD, IBAN_CODE, EMAIL_ADDRESS, IP_ADDRESS (v4+v6), MAC_ADDRESS, CRYPTO_BTC, PHONE_NUMBER — each with ≥3 examples and ≥3 counter_examples
- [x] 4.4 Create `src/catalog/us.toml`: US_SSN, US_ITIN, US_ABA_ROUTING, US_BANK_NUMBER, US_PASSPORT
- [x] 4.5 Create `src/catalog/uk.toml`: UK_NHS, UK_NINO, UK_POSTCODE
- [x] 4.6 Create `src/catalog/es.toml`: ES_NIF, ES_NIE
- [x] 4.7 Create `src/catalog/it.toml`: IT_FISCAL_CODE
- [x] 4.8 Create `src/catalog/pl.toml`: PL_PESEL
- [x] 4.9 Create `src/catalog/sg.toml`: SG_NRIC_FIN
- [x] 4.10 Create `src/catalog/au.toml`: AU_ABN, AU_ACN, AU_TFN
- [x] 4.11 Create `src/catalog/in.toml`: IN_PAN, IN_AADHAAR, IN_GSTIN, IN_VOTER
- [x] 4.12 Create `src/catalog/fi.toml`: FI_HETU

## 5. Detection Engine

- [x] 5.1 Implement `src/engine.rs`: `Engine` struct (recognizers, RegexSet, MaskOptions), `EngineBuilder` with country selection / enable / disable / custom / mask-override methods
- [x] 5.2 Implement `Engine::analyze()`: RegexSet prescreen → per-candidate find_iter → validator gate → push Finding → resolve_overlaps
- [x] 5.3 Implement `Engine::mask()` and `Engine::mask_with()`: build output walking findings, return `Cow::Borrowed` when no findings
- [x] 5.4 Implement `src/lib.rs`: crate-level docs with usage example, pub mod declarations, pub re-exports of Engine / EngineBuilder / Finding / MaskPolicy / MaskOptions / Error / Result, feature cfg for cli/daemon modules

## 6. Config, Daemon & CLI

- [x] 6.1 Implement `src/config.rs`: `Config`, `EngineConfig` (CountrySel custom Deserialize, disable/enable/overrides/custom), `DaemonConfig` (beatInterval, tmpDirectory, maxLinesInOneGo, runMode, jobs), file loaders returning `crate::Result`
- [x] 6.2 Implement `src/daemon/job.rs`: `state_token()` (SHA-256 of path+birthtime, fallbacks), `run_job()` (BufReader skip, BufWriter append, line cap, checkpoint write)
- [x] 6.3 Implement `src/daemon/mod.rs`: `run_once()` iterating all jobs, `run_loop()` with sleep between beats
- [x] 6.4 Implement `src/cli.rs` (feature cli): clap derive `Cli` struct with global `--config`/`-v`, subcommands `mask`/`scan`/`run`/`list` each with `--country`/`--all-countries`
- [x] 6.5 Implement `src/main.rs` (feature cli): parse CLI → load config → init env_logger → dispatch subcommand, exit codes 0/1/2

## 7. Phase 4 — Maximal Catalog

- [x] 7.1 Add DE validator (`de_tax_id`, mod-11 variant) + `src/catalog/de.toml` (DE_TAX_ID ≥3 vectors)
- [x] 7.2 Add FR validator (`fr_nir`, 97 - (n mod 97)) + `src/catalog/fr.toml` (FR_NIR)
- [x] 7.3 Add NL validator (`nl_bsn`, 11-test) + `src/catalog/nl.toml` (NL_BSN)
- [x] 7.4 Add BE validator (`be_nrn`, mod-97) + `src/catalog/be.toml` (BE_NRN)
- [x] 7.5 Add BR validators (`br_cpf`, `br_cnpj`) + `src/catalog/br.toml` (BR_CPF, BR_CNPJ)
- [x] 7.6 Add MX validators (`mx_curp`) + `src/catalog/mx.toml` (MX_CURP, MX_RFC)
- [x] 7.7 Add CA SIN (reuse Luhn) + `src/catalog/ca.toml` (CA_SIN)
- [x] 7.8 Add ZA ID (reuse Luhn) + `src/catalog/za.toml` (ZA_ID)
- [x] 7.9 Add SE personnummer (reuse Luhn) + `src/catalog/se.toml` (SE_PERSONNUMMER)
- [x] 7.10 Add NO validator (`no_fnr`, two mod-11) + `src/catalog/no.toml` (NO_FODSELSNUMMER)
- [x] 7.11 Add DK CPR structural + `src/catalog/dk.toml` (DK_CPR)
- [x] 7.12 Add PT validator (`pt_nif`, mod-11) + `src/catalog/pt.toml` (PT_NIF)
- [x] 7.13 Add IE validator (`ie_pps`, mod-23 letter) + `src/catalog/ie.toml` (IE_PPS)
- [x] 7.14 Add RO validator (`ro_cnp`, weighted mod-11) + `src/catalog/ro.toml` (RO_CNP)
- [x] 7.15 Add TR validator (`tr_tckn`) + `src/catalog/tr.toml` (TR_TCKN)
- [x] 7.16 Add CN validator (`cn_ric`, ISO 7064 MOD 11-2) + `src/catalog/cn.toml` (CN_RIC)
- [x] 7.17 Add JP validator (`jp_mynumber`, mod-11) + `src/catalog/jp.toml` (JP_MYNUMBER)
- [x] 7.18 Add CL validator (`cl_run`, mod-11) + `src/catalog/cl.toml` (CL_RUN)
- [x] 7.19 Add AR validator (`ar_cuit`, mod-11) + `src/catalog/ar.toml` (AR_CUIT, AR_DNI)
- [x] 7.20 Add RU validators (`ru_inn`, `ru_snils`) + `src/catalog/ru.toml` (RU_INN, RU_SNILS)
- [x] 7.21 Add ID NIK structural + `src/catalog/id.toml` (ID_NIK)
- [x] 7.22 Add SWIFT_BIC and CRYPTO_ETH to `src/catalog/global.toml`
- [x] 7.23 Audit all Phase-4 country validators with ≥3 verified sample IDs; defer any unverifiable entry to CHANGELOG

## 8. Tests

- [x] 8.1 Write `tests/catalog.rs`: iterate raw_catalog(), assert ASCII-class patterns (no bare \d/\w), validator name resolves, mask kind valid, ≥3 examples+counter_examples, build single-recognizer Engine per row and assert all vectors
- [x] 8.2 Write `tests/validators.rs`: known-valid and known-invalid vectors for every Validator variant (cite sources in comments)
- [x] 8.3 Write `tests/engine.rs`: multi-entity line, overlap resolution (card beats bank number), each mask strategy exact output, idempotence mask(mask(x))==mask(x), no-PII returns Cow::Borrowed, Unicode line safe
- [x] 8.4 Write `tests/daemon.rs`: tempdir setup, 5-line input with IP+card, maxLinesInOneGo=2, run_once → 2 masked lines + checkpoint==2, second run_once appends next 2 + checkpoint==4; corrupt checkpoint recovery
- [x] 8.5 Write `tests/cli.rs` (assert_cmd): mask stdin card → masked stdout, scan --json returns CREDIT_CARD finding, list prints entities, all exit code 0
- [x] 8.6 Write `tests/proptest.rs`: (a) mask never panics on arbitrary String, (b) embed valid card in random ASCII → output does not contain original AND analyze(output) finds no validated card, (c) output is valid UTF-8
- [x] 8.7 Write `benches/mask.rs` (criterion): throughput on synthetic log corpus, report lines/sec

## 9. Docs, README & Cleanup

- [x] 9.1 Create `docs/README.md` index with one-paragraph overview and explicit no-NER scope note
- [x] 9.2 Create `docs/usage.md`: CLI subcommands, stdin/file examples, daemon setup, systemd unit snippet, RUST_LOG/-v
- [x] 9.3 Create `docs/configuration.md`: pii-masker.json schema reference (engine + daemon keys), country opt-in, examples
- [x] 9.4 Create `docs/recognizers.md`: full catalog table (generate via `pii-masker list --all-countries --json`, document the generation step)
- [x] 9.5 Create `docs/architecture.md`: RE2 constraints, RegexSet prescreen, overlap resolution, mask strategies, country opt-in default, at-least-once daemon, .running removal
- [x] 9.6 Create `docs/adding-recognizers.md`: per-entry contract (TOML schema, mandatory vectors, validator rule, source citation), how to add a new country
- [x] 9.7 Create `docs/security.md`: irreversible masking, hash low-entropy caveat, false-positive tradeoffs, explicit no-NER limitation, not a substitute for at-source tokenization
- [x] 9.8 Create `docs/CONTRIBUTING.md`: dev setup, cargo fmt/clippy/test/bench, recognizer contract, release process (bump version → CHANGELOG → tag → publish)
- [x] 9.9 Rewrite `README.md` to install/quickstart only (cargo install, cargo add --no-default-features, 4 quick-start examples, link to docs/)
- [x] 9.10 Write `CHANGELOG.md` (Keep-a-Changelog): 0.1.0 entry listing shipped entities and any Phase-4 deferrals
- [x] 9.11 Update `AGENTS.md`: replace obsolete architecture section (zero-deps, custom regex engine) with actual module layout, dependency policy, public API (Engine/EngineBuilder), and quality gates
- [x] 9.12 Replace `config/` directory with a single `pii-masker.json` example (engine + daemon keys in new schema); delete old `config/*.json` files
- [x] 9.13 Delete all JavaScript files: `index.js`, `jobs.js`, `utils.js`, `package.json`, `package-lock.json`, `scratch.pad`

## 10. Verification

- [x] 10.1 `cargo build --all-features` passes (binary compiles)
- [x] 10.2 `cargo build --no-default-features` passes (library compiles without CLI deps)
- [x] 10.3 `cargo fmt --all --check` passes
- [x] 10.4 `cargo clippy --all-targets --all-features -- -D warnings` passes
- [x] 10.5 `cargo test --all-features` — all suites green including catalog.rs vectors and proptest no-leak
- [x] 10.6 E2E: `printf 'pay 4111 1111 1111 1111 from 10.0.0.1, pan ABCDE1234F, ssn 219-09-9999\n' | cargo run -- mask --country IN,US` — card → `**** **** **** 1111`, IP → `[IP_ADDRESS]`, PAN → `[IN_PAN]`, SSN → `[US_SSN]`; invalid-Luhn card passes through unmasked
- [x] 10.7 `pii-masker list --all-countries --json` count matches `docs/recognizers.md`
- [x] 10.8 `cargo publish --dry-run` packages cleanly; confirm `pii-masker` name available before actual publish
- [x] 10.9 Confirm no `*.js`, `package*.json`, or `scratch.pad` remain in the tree
