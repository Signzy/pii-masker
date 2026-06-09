# Project: PII Masker

Banking-grade Personally Identifiable Information detection and masking library.
Published on crates.io. Zero dependencies beyond the Rust standard library.

## Domain

High-throughput log and document sanitization for regulated environments (banking,
finance, healthcare). Must never leak PII — correctness over speed, but still fast
enough to keep up with production log generation (~200+ lines/sec on modest hardware).

## Scope

- **Input formats**: plain text lines, JSON payloads, key=value pairs, URL query strings,
  and common log formats (Apache, Nginx, syslog, JSON-lines).
- **Output**: same structure, with detected PII replaced by masked tokens preserving
  partial visibility where configured (e.g. first 4 + last 4 digits of a card number).
- **Matchers**: regex patterns, checksum-validated numbers (Luhn for credit cards,
  mod-97 for IBANs), structural parsers for formats where regex alone is insufficient.

## PII Categories

### Global identifiers
| Category | Examples |
|---|---|
| Credit / debit cards | PAN (Luhn-validated), track data |
| Government IDs | SSN (US), Aadhaar (IN), PAN (IN), NIN (UK), SIN (CA), TFN (AU), CPF (BR), etc. |
| Tax IDs | EIN, VAT, GST, etc. |
| Passport numbers | Country-specific formats |
| Driver's licenses | Country/state-specific formats |
| IBAN / SWIFT / BIC | Bank account identifiers with checksum validation |
| Phone numbers | E.164, national formats per country |
| Email addresses | RFC 5322 |
| IP addresses | IPv4, IPv6 |
| MAC addresses | EUI-48, EUI-64 |
| Dates of birth | Multiple formats |
| Physical addresses | Multi-line, structured |
| Vehicle plates | Country-specific |

### Presets
Pre-built matcher sets per country / region:
- `IN` (India) — Aadhaar, PAN, GSTIN, driving licence, voter ID, Indian phone, etc.
- `US` — SSN, EIN, state driver's licences, US phone, etc.
- `UK` — NINO, NHS number, UK phone, driving licence, etc.
- `EU` — IBAN per country, VAT IDs, etc.
- `global` — email, IP, MAC, credit cards, dates, common regex patterns
- `all` — every matcher available

Users compose presets: `["global", "IN", "US"]` gives them global PII + India + US
specifics.

## Architecture

### Dependency policy
**Zero third-party dependencies.** Everything built on `std`:
- No `regex` crate — custom NFA/DFA regex engine (or hand-written scanners per format)
- No `serde` — manual JSON/TOML parsing for config files
- No `log` / `env_logger` — `eprintln!` / `std::io::stderr` for diagnostics
- No external crypto — checksum algorithms (Luhn, mod-97, Verhoeff, Damm, etc.)
  implemented inline from spec

### Crate structure
```
src/
├── lib.rs              # Public API: Masker::new(presets), masker.mask(&str)
├── masker.rs           # Orchestrator: dispatch input to format parser, apply matched rules
├── config.rs           # Preset definitions, rule loading, user overrides
├── engine/
│   ├── mod.rs          # Scanner trait + dispatch
│   ├── regex.rs        # Custom NFA/DFA regex engine
│   ├── luhn.rs         # Luhn checksum (credit cards, IMEI)
│   ├── iban.rs         # IBAN mod-97 validation + per-country BBAN extraction
│   └── verhoeff.rs     # Verhoeff / Damm checksums (Aadhaar, etc.)
├── matchers/
│   ├── mod.rs          # Matcher trait: detect + mask
│   ├── credit_card.rs
│   ├── aadhaar.rs
│   ├── pan_india.rs
│   ├── ssn_us.rs
│   ├── phone.rs
│   ├── email.rs
│   ├── ip.rs
│   ├── iban.rs
│   ├── passport.rs
│   └── ...             # One file per PII category or country set
├── format/
│   ├── mod.rs          # FormatDetector trait
│   ├── plain.rs        # Plain text (stream line by line)
│   ├── json.rs         # Recursive JSON value walk
│   ├── kv.rs           # key=value pairs
│   └── url.rs          # URL query string decode + mask
├── presets/
│   ├── mod.rs          # Preset registry
│   ├── global.rs
│   ├── india.rs
│   ├── us.rs
│   ├── uk.rs
│   └── eu.rs
└── progress.rs         # File-based resume tracking (linenumber, running state)
```

### Public API (sketch)
```rust
use pii_masker::{Masker, Preset};

fn main() {
    let masker = Masker::builder()
        .presets(&[Preset::Global, Preset::India])
        .partial_visibility(4, 4)  // keep first 4 + last 4 characters visible
        .build()
        .unwrap();

    let masked = masker.mask_line("user pan is ABCDE1234F and IP 192.168.1.1");
    // → "user pan is ABCDXX-PAN-XX4F and IP 192.XXX.XXX.XXX"
}
```

### Distribution

- **crates.io** — primary distribution for the Rust library and CLI binary.
- **npm** — JavaScript/TypeScript package published via WASM (`wasm-pack`), exposing
  the same masking engine to Node.js and browser runtimes with zero native deps.

### CLI binary

Ship a binary that reads files, applies masking, writes output, and resumes from checkpoints — file-based progress tracking with configurable beat interval.

## Testing

- **Unit tests** per matcher: valid samples, invalid samples, edge cases, boundary values
- **Checksum validation tests**: generated valid numbers, common typos, transposition errors
- **Integration tests**: golden files (input → expected output), multi-format pipelines
- **Fuzz testing**: random byte sequences must never panic; use `cargo fuzz` with custom harnesses
- **Property-based tests**: masking is idempotent (masking masked output produces same output)
- **Benchmarks**: `cargo bench` for throughput (lines/sec) on realistic log samples
- **Coverage**: `cargo tarpaulin` or `cargo llvm-cov`, target ≥95% line coverage

## Quality gates

Run `./ci-check.sh` before every commit. It gates on:

| Gate | Threshold | Enforced by |
|---|---|---|
| Formatting | `rustfmt.toml` | `cargo fmt --check` |
| Clippy lints | zero warnings (pedantic + nursery) | `cargo clippy` |
| Cognitive complexity | ≤ 25 per function | `clippy.toml` |
| Cyclomatic complexity | ≤ 25 per function | `clippy.toml` |
| Lines per function | ≤ 200 | `clippy.toml` |
| Lines per file | ≤ 500 | `ci-check.sh` |
| Arguments per function | ≤ 6 | `clippy.toml` |
| Type complexity | ≤ 250 | `clippy.toml` |
| Trait bounds | ≤ 5 per type param | `clippy.toml` |
| Unsafe code | forbidden by default | `ci-check.sh` audit |
| Stack types | ≤ 512 bytes | `clippy.toml` |
| Tests | all pass | `cargo test` |
| Docs | no broken links | `cargo doc` |

Auto-fix formatting: `./ci-check.sh --fix`

## Conventions

- 4-space indentation, 100-char line width (`rustfmt.toml`)
- Error handling: custom `enum Error` with `Display` + `std::error::Error`, no `.unwrap()` in library code
- Logging: `eprintln!` for CLI diagnostics, library code returns `Result` and stays silent
- No panics from malformed input — return `Err`
- All public types: `#[must_use]` where appropriate, doc comments with examples
