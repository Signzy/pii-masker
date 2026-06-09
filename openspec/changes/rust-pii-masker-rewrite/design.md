## Context

The repo currently ships a Node.js daemon (`index.js`, `jobs.js`, `utils.js`) that reads log files line-by-line, applies plain regex replacements from a JSON rules file, and writes masked output. It has no checksum validation, no structured recognizer catalog, no country packs, and requires Node.js at runtime.

The replacement is a Rust library crate (`pii_masker`) + binary (`pii-masker`) that is:
- Published on crates.io
- Banking-grade: every detected identifier is checksum-validated before masking
- Data-driven: recognizers live in TOML catalog files, not compiled code
- Multi-country: global (CREDIT_CARD, IBAN, EMAIL, IP, MAC, PHONE) + country packs (US, UK, ES, IT, PL, SG, AU, IN, FI, and 20+ Phase-4 countries)
- CLI + daemon: same beat-loop daemon semantics, now as a `run` subcommand

## Goals / Non-Goals

**Goals:**
- Replace all JavaScript with idiomatic Rust; zero Node runtime requirement
- Checksum-validated detection: Luhn (cards), IBAN mod-97, Verhoeff (Aadhaar), NHS mod-11, and 20+ country-specific algorithms
- Data-driven recognizer catalog: adding a country = adding a TOML file + test vectors, no engine code changes
- Smart masking per entity: Partial (card last-4), Redact (SSN/passports), Hash (deterministic audit token)
- Country packs opt-in: global entities always on; country packs enabled via config `countries` array
- Single crate, two targets: `cargo add pii-masker --no-default-features` pulls only the engine (no CLI deps)
- Embedded test vectors: every catalog row auto-tested by `tests/catalog.rs`; a recognizer cannot ship without passing vectors
- Original daemon resumability preserved: SHA-256 state token, `.linenumber` checkpoint, at-least-once semantics
- Published to crates.io (GPL-3.0-only; name `pii-masker`)

**Non-Goals:**
- Free-text NER: no PERSON, LOCATION, ADDRESS, DATE_TIME, NRP detection. No ML/ONNX/tokenizer dependencies.
- Reversible tokenization or vault: masking is irreversible by design
- Format-aware parsing (JSON field walking, URL query decode): plain-text line masking only
- WASM/npm package
- Regex engine from scratch: the `regex` crate (RE2) is used

## Decisions

**D1: Use the `regex` crate (RE2), not a custom NFA.**
The original AGENTS.md specified zero dependencies and a custom regex engine. RE2 is safe (no catastrophic backtracking), well-maintained, and already handles Unicode correctly. Building a custom NFA would be months of work with no correctness advantage for structured ID patterns. Trade-off: adds one dependency; mitigated by RE2's strong security track record.

**D2: ASCII character classes (`[0-9]`, `[A-Za-z0-9]`) everywhere in patterns.**
Rust's `regex` crate makes `\d` and `\w` Unicode-aware. A Unicode `\d` matches Devanagari digits, which would produce false positives in Aadhaar/PAN patterns. All catalog patterns use ASCII classes to stay precise. Exclusion logic (e.g. rejecting leading zeros) lives in validators, not patterns — keeping patterns loose.

**D3: `RegexSet` prescreen before per-recognizer extraction.**
Running N individual regexes on every line is O(N × line_length). `RegexSet` runs all patterns in one DFA pass and returns the set of matching pattern indices. Only those recognizers then run `find_iter`. For a 60-recognizer catalog this is a meaningful speedup on clean lines.

**D4: Validators as a named registry, not a trait object.**
A `Validator` enum with a `by_name(&str)` registry lets the TOML catalog reference validators by string (`"luhn"`, `"iban"`, `"verhoeff"`) without dynamic dispatch. All validator functions are pure: `validate(&str) -> bool`. New validators are added by extending the enum and the `by_name` match arm — one place, no trait impls or Box allocations.

**D5: OnceLock for catalog parse.**
`raw_catalog()` parses all TOML once at first call and returns a `&'static [RawRecognizer]`. Subsequent `Engine::builder()` calls are cheap. Avoids re-parsing on every test or CLI invocation.

**D6: `cli` feature gates all CLI deps.**
`cargo add pii-masker --no-default-features` pulls only `regex`, `serde`, `serde_json`, `toml`, `sha2`, `log`, `thiserror`. Libraries embedding pii-masker do not pay for clap/anyhow/env_logger. The `[[bin]]` entry carries `required-features = ["cli"]` so the binary is not built without the feature.

**D7: Country packs opt-in, global on by default.**
Country-specific recognizers (US_SSN, UK_NHS, IN_AADHAAR, …) are disabled unless the config/CLI opts them in. Rationale: a US_BANK_NUMBER pattern (8–17 digits, score 0.3) applied globally produces massive false positives on non-US log lines. Global entities (CREDIT_CARD, IBAN, EMAIL, IP, MAC, PHONE) have tight enough patterns to be safe everywhere.

**D8: SHA-256 state token for daemon, not MD5.**
The original JS used `crypto.createHash('md5')`. SHA-256 is used everywhere (state token + Hash mask strategy) via the same `sha2` dep already in the tree. MD5 is not added.

**D9: Remove `.running` lock file.**
The JS `.running` mechanism was vestigial (always `0`, never actually checked for concurrency). Removing it simplifies the daemon and avoids stale lock files after crashes.

**D10: At-least-once checkpoint semantics.**
The `.linenumber` file is written after each line in the JS (per-line `writeFileSync`). The Rust daemon writes it once after the batch (`max_lines_in_one_go`) completes. On crash, the last completed batch is re-processed (at-least-once). This is the correct trade-off for log masking: a duplicate output line is far less bad than a missed PII token.

## Risks / Trade-offs

**Risk: Catalog correctness is gated by embedded vectors, not code review.**
Every recognizer row ships with ≥3 examples and ≥3 counter_examples. `tests/catalog.rs` builds a single-recognizer Engine per row and asserts all examples detect+validate, all counter_examples produce no finding. A wrong regex or checksum that passes vectors will still ship — but vectors are the best available gate without external data sets.
*Mitigation*: Cite authoritative sources (Presidio, Wikipedia national ID specs, official government docs) in TOML comments. Mark unverified checksums with `validator = "none"` + reduced score rather than shipping a wrong algorithm.

**Risk: PHONE_NUMBER false positives.**
The default phone recognizer is `\b\+?[0-9][0-9\-\s().]{6,16}[0-9]\b` with `phone_loose` validator (7–15 digit count). This will match date strings, part numbers, and other numeric sequences. Score is 0.4 (low) to reflect this.
*Mitigation*: Strict phone validation available via `--features phone` (`phonenumber` crate). Users sensitive to false positives should enable this feature or disable PHONE_NUMBER via config.

**Risk: Low-score context-dependent entities (US_BANK_NUMBER, US_PASSPORT) overlap.**
Both are `[0-9]{9}`. US_PASSPORT score 0.3, US_ABA_ROUTING score via `aba_routing` validator. A 9-digit number that passes ABA routing checksum will be detected as US_ABA_ROUTING; one that doesn't passes through as US_PASSPORT (if context words present).
*Mitigation*: Overlap resolution: higher score wins. Context words boost score (future work — Phase 4+).

**Risk: `created()` birth time unsupported on some Linux filesystems.**
The daemon state token uses `metadata.created()` for log-rotation detection. On ext4 without `inode_creation_time`, this returns an error.
*Mitigation*: Fall back to `metadata.modified()`. If both fail, use path-only token (rotation restart disabled) and log a warning. Documented in `docs/security.md`.

**Risk: Hash mask is deterministic and low-entropy inputs are rainbow-table reversible.**
A 4-digit PIN hashed to `[PIN:a3f2b1c4d5e6]` can be reversed with a precomputed table.
*Mitigation*: Document clearly in `docs/security.md`. Hash strategy is intended for audit-trail correlation, not for high-entropy secrets like full SSNs where an attacker would need to brute-force 9 digits.

**Risk: crate name `pii-masker` may be taken on crates.io.**
Not verified at plan time.
*Mitigation*: Fallback name `pii-sanitizer`. Check before `cargo publish`; update `name` in Cargo.toml, binary name, and docs if needed.
