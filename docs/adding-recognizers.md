# Adding recognizers

Adding a new country or entity to the catalog requires **no engine changes** — only
data changes: a TOML file entry and (if needed) a new validator function.

## Per-entry contract

Every `[[recognizer]]` entry in `src/catalog/*.toml` MUST satisfy:

1. **ASCII-class regex**: use `[0-9]`, `[A-Za-z]`, `[A-Za-z0-9]`. Never bare `\d`,
   `\w`, or `\s` — they are Unicode-aware in Rust's `regex` crate and produce false
   positives on non-ASCII scripts.

2. **Loose pattern, tight validator**: the regex matches a broad class; all
   exclusion logic lives in the validator. No negative look-ahead (RE2 doesn't
   support it anyway).

3. **Word boundaries** (`\b`) on both ends to avoid substring matches.

4. **≥ 3 `examples`**: each must match the regex AND pass the validator. Use real
   published sample IDs from the authoritative source.

5. **≥ 3 `counter_examples`**: strings that must NOT produce a validated finding.
   Include at least one that matches the regex but fails the validator (to test the
   validator, not just the regex).

6. **Source citation**: a TOML comment above the entry citing the authoritative
   source (Presidio recognizer page, Wikipedia national ID article, official
   government specification).

7. **Validator rule**: if a checksum cannot be verified against ≥ 3 published valid
   samples, set `validator = "none"` and reduce `score` rather than shipping a
   guessed algorithm.

## TOML schema

```toml
# Source: <URL to authoritative spec>
[[recognizer]]
entity = "XX_ENTITY_NAME"           # SCREAMING_SNAKE_CASE, country prefix for country packs
country = "XX"                      # ISO-3166-1 alpha-2; "" for global entities
regex = '\b[0-9]{9}\b'              # single-quoted raw string (no double-escaping)
validator = "validator_name"        # registered in Validator::by_name()
score = 0.85                        # 0.0–1.0; lower for loose/context-dependent recognizers
context = ["word1", "word2"]        # context words for future context-scoring (informational now)
enabled = true
examples = ["valid1", "valid2", "valid3"]
counter_examples = ["bad1", "bad2", "bad3"]
mask = { kind = "redact" }          # or partial/hash; see docs/configuration.md
```

## Adding a validator

1. Add a new variant to the `Validator` enum in `src/validator.rs`.
2. Add the variant to `Validator::by_name()` with a snake_case name.
3. Add a match arm in `Validator::validate()` calling the algorithm function.
4. Implement the algorithm as a private `fn` with its own `#[test]` citing
   authoritative vectors.
5. Update `Validator::has_checksum()` if the new validator applies a checksum.
6. Verify ≥ 3 published valid sample IDs against your implementation before
   referencing the validator from TOML.

## Deferring unverified recognizers

If a country format exists in the plan but cannot be verified to the per-entry
contract at release time, the **correct action** is:

- Set `validator = "none"` and reduce `score` (to 0.50–0.70)
- Note the deferral in `CHANGELOG.md` under the relevant version
- Do NOT ship a guessed checksum — a wrong validator produces silent false negatives
  (PII passes through unmasked when the algorithm rejects the correct value)

## Running catalog tests

After adding a new entry, the catalog self-test runs automatically:

```sh
cargo test --test catalog
```

This builds a single-recognizer engine per catalog row and asserts:
- All `examples` are detected + validated.
- All `counter_examples` produce zero findings.
- No bare `\d`/`\w` in patterns.
- ≥ 3 examples and ≥ 3 counter_examples per entry.
