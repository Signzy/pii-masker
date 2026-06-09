# Security notes

## Irreversibility

All three masking strategies are **irreversible**:
- `Redact` replaces the value with a labeled token — information is destroyed.
- `Partial` removes most characters — the full value cannot be recovered.
- `Hash` replaces with a SHA-256 token — recovering the original requires knowing
  the input, which defeats the purpose for high-entropy values.

There is no vault, no encryption key, and no tokenization round-trip. This is
deliberate: a compromised masked log must not be reversible without the original
data source.

## Deterministic hash — low-entropy caveat

The `Hash` strategy computes `SHA-256(matched_bytes)` and includes the first 12 hex
characters in the output token. This is **deterministic**: the same value always
produces the same token, which is useful for correlating masked records across logs.

**Caveat**: for low-entropy inputs (e.g. short PINs, common short IDs), the hash is
vulnerable to **rainbow-table attacks** — a precomputed table of all possible inputs
can reverse the hash in milliseconds.

Use `Hash` only for high-entropy values (full card numbers, IBANs, UUIDs) where
brute-force is infeasible. For low-entropy fields, use `Redact` instead.

## Precision vs. recall

- **Low-score, context-dependent entities** (US_BANK_NUMBER score 0.30, US_PASSPORT
  score 0.30, DK_CPR `none` validator) produce more false positives. Enable them only
  when you know the log source contains such identifiers.
- **PHONE_NUMBER** (score 0.40, loose validator) will match date strings, part
  numbers, and serial numbers. For strict phone validation, build with
  `--features phone` (`phonenumber` crate).
- **Country packs are opt-in** precisely because false positives compound. A German
  DE_TAX_ID pattern (`[1-9][0-9]{10}`) will match any 11-digit integer in any log.

## Explicit non-scope: no free-text NER

This library detects **structured, checksum-validated identifiers only**. It does
**not** detect:
- Person names (PERSON)
- Physical addresses (LOCATION, ADDRESS)
- Dates of birth (DATE_TIME)
- Nationalities, religions, political beliefs (NRP)

These require named-entity recognition (NER) with a trained model. Adding ML/ONNX/
tokenizer dependencies is **explicitly out of scope** for this crate. If you need
NER-based detection, use a separate NLP pipeline (e.g. spaCy, Hugging Face) before
or after this library.

## Not a substitute for at-source tokenization

Masking log output does not protect data that has already been transmitted unmasked,
stored in databases, indexed by search engines, or retained in memory dumps.
This library is a **best-effort log sanitizer** — it is not a substitute for:
- Tokenizing PII at the point of capture
- Encrypting data at rest and in transit
- Access controls on raw log storage
- Audit logging of who accessed what

## At-least-once semantics

The daemon checkpoint is written **after** each batch completes. On crash, the last
batch is re-processed, which may produce duplicate output lines. This is intentional:
a duplicate masked line is a far better outcome than a gap that causes a PII token
to slip through unmasked into the output log.

If strict exactly-once semantics are required, run the daemon with
`--once` behind a process supervisor that enforces single execution, and handle
output deduplication externally.
