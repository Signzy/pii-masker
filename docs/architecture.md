# Architecture

## RE2 constraints and pattern design

The `regex` crate implements RE2, which guarantees linear-time matching and is safe
against catastrophic backtracking. RE2 has **no look-ahead, no look-behind, and no
backreferences** — features common in Python/PCRE patterns that produce O(n²)
performance in adversarial inputs.

Consequences for the catalog:
- Patterns are **deliberately loose** — they match a broad class of strings.
- All exclusion logic lives in **validators** (checksum functions), not regex.
- ASCII character classes everywhere: `[0-9]` not `\d`, `[A-Za-z0-9]` not `\w`.
  Rust's `regex` crate makes `\d`/`\w` Unicode-aware; `\d` would match Devanagari
  digits and cause false positives in Aadhaar/PAN patterns.

## Detection pipeline

```
Input text
    │
    ▼
RegexSet::matches(text)          ← one DFA pass over all N patterns
    │
    ├─ candidate set {i₁, i₂, …} ← only patterns that matched
    │
    ▼
for each iₙ:
  recognizer[iₙ].regex.find_iter(text)
      │
      ▼
  for each span m:
    validator.validate(&text[m.range()])
      │
      ├─ pass → push Finding(entity, start, end, score)
      └─ fail → drop silently
    │
    ▼
resolve_overlaps(hits)           ← sort + greedy keep
    │
    ▼
findings: Vec<Finding>
```

`RegexSet` runs all patterns in one DFA pass and returns the matching pattern
indices. Only those recognizers then run `find_iter`. For a 60-recognizer catalog
this avoids N DFA constructions on every clean line.

## Overlap resolution

Sort findings by: **(start asc, score desc, len desc, entity asc)**. Greedily keep
a finding when its `start >= last_end`. This gives precedence to:
1. Earlier spans (lower start wins)
2. Higher-confidence recognizers (score desc)
3. Longer matches (len desc, tie-breaker)
4. Lexicographic entity name (determinism for equal scores)

## Masking strategies

| Strategy | Output | Use case |
|---|---|---|
| `Partial { prefix, suffix, mask_char }` | `**** **** **** 1111` | Card last-4, IBAN partial |
| `Redact { label }` | `[US_SSN]` | Full redaction |
| `Hash { label }` | `[CREDIT_CARD:a3f2b1c4d5e6]` | Deterministic audit token |

`Hash` uses SHA-256 of the matched bytes; the first 12 hex characters are included
in the token. This is **deterministic** — the same value always produces the same
token — useful for correlating audit trails across masked logs.

## Country pack opt-in

Country packs are **disabled by default** to minimise false positives. A
`US_BANK_NUMBER` pattern (`[0-9]{8,17}`, score 0.30) applied globally would match
dates, part numbers, and serial numbers in non-US log lines. Global entities
(CREDIT_CARD, IBAN_CODE, EMAIL_ADDRESS, etc.) have tight enough patterns and
high-entropy validation to be safe everywhere.

Opt-in via `--country US,IN` or `"countries": ["US","IN"]` in config.

## Catalog data model

Each `[[recognizer]]` entry in a TOML file compiles to a `Recognizer` struct:
- `regex: Regex` — compiled RE2 pattern
- `validator: Validator` — checksum function (or `None`)
- `mask: MaskPolicy` — resolved from `RawMask`
- `examples/counter_examples` — embedded test vectors (tested automatically)

The catalog is parsed once at first use via `OnceLock<Vec<RawRecognizer>>` and
`recognizers()` compiles all entries. Subsequent `Engine::builder()` calls are cheap.

## Daemon checkpoint semantics

The state token for each job is:
```
hex(SHA256(path + birth_time_secs))[..32]
```
Using birth time (falling back to modified time, then path-only) detects log rotation:
when a file is replaced, the token changes and the daemon restarts from line 0.

Checkpoint write happens **after** the batch completes (at-least-once). On crash,
lines from the last incomplete batch are re-processed. A duplicate output line is
a far better outcome than a missed PII token.

The vestigial `.running` lock file from the original Node.js implementation is
**removed** — it was never actually checked for concurrency.
