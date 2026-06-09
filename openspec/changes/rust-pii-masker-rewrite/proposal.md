## Why

The project currently ships a Node.js log-sanitizer that applies plain regex replacements without checksum validation, producing false positives and missing structurally-valid PII that fails a simple pattern match. Replacing it with a Rust library gives checksum-validated detection, a data-driven multi-country recognizer catalog, and a crates.io-publishable crate with a resumable daemon binary — all while eliminating the Node runtime dependency.

## What Changes

- **BREAKING**: All JavaScript source files removed (`index.js`, `jobs.js`, `utils.js`, `package.json`, `package-lock.json`, `scratch.pad`).
- **BREAKING**: `config/` directory replaced with a single `pii-masker.json` example in the new schema.
- New Rust crate `pii-masker` (lib `pii_masker`) at crate root — `src/` entirely new.
- Library crate (no CLI deps by default) + `cli` feature that gates `clap`, `env_logger`, `anyhow`.
- Optional `phone` feature for strict phone validation via the `phonenumber` crate.
- Multi-country PII detection: Global, US, UK, ES, IT, PL, SG, AU, IN, FI, and 20+ more country packs (Phase 4), each with checksum-validated identifiers and embedded test vectors.
- Three masking strategies: `Partial` (keep prefix/suffix alnum), `Redact` (labeled token), `Hash` (deterministic SHA-256 12-hex token).
- `RegexSet` prescreen → per-recognizer span extraction → validator gate → overlap resolution pipeline.
- Country packs opt-in by default (global identifiers always on) to minimise false positives.
- Four CLI subcommands: `mask`, `scan`, `run` (daemon), `list`.
- Resumable log-stream daemon: state token per file, beat interval, at-least-once checkpoint.
- `docs/` directory with usage, configuration, architecture, security, and contributing guides.
- `CHANGELOG.md`, updated `README.md` (install/quickstart only), updated `AGENTS.md`.

## Capabilities

### New Capabilities

- `pii-detection-engine`: Core detection pipeline — `RegexSet` prescreen, per-recognizer `find_iter`, validator gate, `Finding` structs, overlap resolution by score/length/entity. Public `Engine` + `EngineBuilder` API.
- `checksum-validators`: All country-specific checksum algorithms (Luhn, IBAN mod-97, Verhoeff for Aadhaar, NHS mod-11, US SSN structural, ES NIF/NIE, IT fiscal code, PL PESEL, AU ABN/ACN/TFN, SG NRIC, GSTIN, ABA routing, FI HETU, and 20+ Phase-4 validators). Each registered by name for use from TOML catalog.
- `masking-strategies`: `MaskPolicy` enum (`Partial`, `Redact`, `Hash`) with `MaskOptions`, `apply_mask()`. SHA-256 hashing via `sha2`, no vault/tokenization.
- `recognizer-catalog`: Embedded TOML files per country pack (`src/catalog/*.toml`), parsed once via `OnceLock`. Each recognizer carries `examples` and `counter_examples` that are automatically exercised by `tests/catalog.rs`. Country packs opt-in via config.
- `cli`: `clap`-derive CLI behind `cli` feature. Subcommands: `mask` (stream stdin/file), `scan` (findings report, `--json`), `run` (daemon), `list` (catalog, `--json`). Global `--config` and `-v/--verbose` flags.
- `log-stream-daemon`: Resumable log masking daemon (`run` subcommand). State token from file birth/modified time + path hash (SHA-256). Per-job `.linenumber` file tracks progress. Beat interval, `maxLinesInOneGo` cap, append-mode output, at-least-once on crash. `.running` lock removed.

### Modified Capabilities

_None — no existing spec-level capabilities exist; this is a full rewrite._

## Impact

- **Removed runtime dep**: Node.js / npm no longer required.
- **New build dep**: Rust stable ≥ 1.74 (edition 2021).
- **New lib deps**: `regex`, `serde`+`serde_json`, `toml 0.8`, `sha2 0.10`, `log`, `thiserror`; CLI adds `clap 4`, `env_logger 0.11`, `anyhow`.
- **License**: GPL-3.0-only (matches existing `LICENSE` file verbatim).
- **Config schema change**: old `config/master.json`, `config/jobs.json`, `config/rules-*.json` replaced by single `pii-masker.json` with `engine` + `daemon` top-level keys.
- **Binary name**: `pii-masker` (same as before, now a Rust binary installed via `cargo install pii-masker`).
- **`AGENTS.md`**: Must be updated to reflect the new module layout, dependency policy, and public API.
- **CI**: `.github/workflows/ci.yml` and `release.yml` replace any Node-based CI; gates on `cargo fmt`, `cargo clippy -D warnings`, `cargo test --all-features`, `cargo build --release`.
