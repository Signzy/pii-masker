# Project: pii-masker

Banking-grade PII detection and masking library in Rust. Published on crates.io
as `pii-masker` (lib crate `pii_masker`).

## Domain

High-throughput log and document sanitisation for regulated environments (banking,
finance, healthcare). Detects and masks checksum-validated structured identifiers
— credit cards, IBANs, national IDs, tax numbers — across 30+ countries.

**Scope limit**: structured identifiers only. No free-text NER (no PERSON, LOCATION,
ADDRESS, DATE_TIME, NRP). No ML/ONNX/tokenizer dependencies. See `docs/security.md`.

## Tech stack

- **Language**: Rust, edition 2021, `rust-version = "1.74"`
- **Key deps**: `regex` (RE2), `serde`+`serde_json`, `toml 0.8`, `sha2 0.10`, `log`, `thiserror`
- **CLI deps** (feature `cli`, default): `clap 4`, `env_logger 0.11`, `anyhow`
- **Optional**: `phonenumber 0.3` (feature `phone`, for strict phone validation)
- **Dev**: `tempfile`, `proptest`, `assert_cmd`, `predicates`, `criterion`
- **License**: GPL-3.0-only

## Module layout

```
src/
  lib.rs            crate docs, pub re-exports, feature cfg
  error.rs          Error (thiserror) + Result
  validator.rs      Validator enum + by_name() registry + all checksum algorithms
  mask.rs           MaskPolicy (Partial/Redact/Hash), MaskOptions, apply_mask()
  recognizer.rs     RawRecognizer (serde/TOML) + compiled Recognizer
  catalog.rs        include_str! TOML files, OnceLock parse, raw_catalog() + recognizers()
  finding.rs        Finding struct + resolve_overlaps()
  engine.rs         Engine, EngineBuilder (country selection, enable/disable, custom)
  config.rs         Config { engine: EngineConfig, daemon: DaemonConfig } + loaders
  daemon/
    mod.rs          run_once(), run_loop()
    job.rs          state_token(), run_job(), JobStats
  cli.rs            clap derive Cli + Cmd (feature "cli")
  main.rs           binary entry (feature "cli")
src/catalog/        data: global.toml, us.toml, uk.toml, es.toml, it.toml,
                    pl.toml, sg.toml, au.toml, in.toml, fi.toml, de.toml, fr.toml,
                    nl.toml, be.toml, br.toml, mx.toml, ca.toml, za.toml, se.toml,
                    no.toml, dk.toml, pt.toml, ie.toml, ro.toml, tr.toml, cn.toml,
                    jp.toml, cl.toml, ar.toml, ru.toml, id.toml
tests/              integration tests (catalog.rs, validators.rs, engine.rs, daemon.rs, cli.rs, proptest.rs)
benches/            criterion bench (mask.rs)
docs/               usage, configuration, architecture, security, adding-recognizers, CONTRIBUTING
```

## Public API

```rust
use pii_masker::{Engine, EngineBuilder};

// Global recognizers (credit cards, IBAN, email, IP, MAC, phone)
let engine = Engine::builder().build()?;
let masked = engine.mask("pay 4111 1111 1111 1111");
// returns Cow<str> — Borrowed if no PII found

// With country packs
let engine = EngineBuilder::default()
    .countries(["US", "IN"])
    .build()?;
let findings = engine.analyze("ssn 219-09-9999, pan ABCDE1234F");
// returns Vec<Finding> { entity, country, start, end, score }
```

## Detection pipeline

```
RegexSet prescreen (one DFA pass)
  → per-candidate find_iter
    → validator gate (checksum / structural)
      → resolve_overlaps (score desc, len desc, entity asc)
        → Vec<Finding>
```

## Catalog authoring rules

- ASCII character classes only: `[0-9]`, `[A-Za-z]`, `[A-Za-z0-9]` — never `\d`/`\w`
- Patterns are loose; exclusion lives in validators
- Every `[[recognizer]]` entry has ≥3 `examples` and ≥3 `counter_examples`
- `validator = "none"` + reduced score if checksum unverified (never ship guessed algo)
- Source citation in TOML comment

## Quality gates

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --no-default-features          # lib without CLI deps
```

Run `./ci-check.sh` for the full gate set including file-size limits.

## Conventions

- Error handling: `enum Error` via `thiserror`; no `.unwrap()` in library code
- Logging: `log::` macros in library; `env_logger` in binary only
- No panics from malformed input; return `Err`
- `#[deny(unsafe_code)]` crate-wide (via `[lints.rust]` in Cargo.toml)
- All public items: `///` doc comments required (`missing_docs = "warn"`)
