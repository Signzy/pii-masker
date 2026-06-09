## ADDED Requirements

### Requirement: Partial masking
The `Partial { prefix, suffix, mask_char }` policy MUST keep the first `prefix` and last `suffix`
alphanumeric characters visible, replacing all other alnum characters with `mask_char`. Non-alnum
separators (spaces, dashes, dots) are preserved verbatim in their original positions.

#### Scenario: Card last-4
- **WHEN** `Partial { prefix: 0, suffix: 4, mask_char: '*' }` is applied to `"4111 1111 1111 1111"`
- **THEN** output is `"**** **** **** 1111"`

#### Scenario: Prefix and suffix both set
- **WHEN** `Partial { prefix: 2, suffix: 2, mask_char: 'X' }` is applied to `"AB123456CD"`
- **THEN** output is `"ABXXXXXXCD"`

#### Scenario: Separator preservation
- **WHEN** applied to `"4111-1111-1111-1111"` with suffix 4
- **THEN** hyphens are preserved: `"****-****-****-1111"`

#### Scenario: prefix + suffix >= alnum count
- **WHEN** `prefix + suffix >= total alnum chars`
- **THEN** all alnum characters are visible (nothing to mask; string returned as-is)

### Requirement: Redact masking
The `Redact { label }` policy MUST replace the entire matched span with `opts.redact_template`
with `{entity}` substituted by `label`.

#### Scenario: Default template
- **WHEN** label is `"US_SSN"` and opts uses the default template `"[{entity}]"`
- **THEN** output is `"[US_SSN]"`

#### Scenario: Custom template
- **WHEN** `redact_template` is `"<REDACTED:{entity}>"`
- **THEN** output for label `"US_SSN"` is `"<REDACTED:US_SSN>"`

### Requirement: Hash masking
The `Hash { label }` policy MUST replace the matched span with `[LABEL:HHHHHHHHHHHH]` where
HHHHHHHHHHHH is the first 12 lowercase hex characters of the SHA-256 digest of the matched bytes.

#### Scenario: Deterministic output
- **WHEN** the same input string is hashed twice
- **THEN** the output token is identical both times

#### Scenario: Different inputs differ
- **WHEN** two distinct card numbers are hashed
- **THEN** their 12-hex tokens differ with overwhelming probability (collision = 1 in 2^48)

#### Scenario: Format
- **WHEN** label is `"CREDIT_CARD"` and matched is `"4111111111111111"`
- **THEN** output matches the pattern `\[CREDIT_CARD:[0-9a-f]{12}\]`

### Requirement: MaskOptions default
`MaskOptions::default()` MUST produce `redact_template = "[{entity}]"`. The struct must be `Clone`.

### Requirement: Irreversibility
All three strategies MUST be irreversible. The original value cannot be recovered from the masked
output without brute-force. No vault, no tokenization, no encryption key.

### Requirement: Label defaults to entity name
When a catalog entry does not override the mask label, the entity name (e.g. `"CREDIT_CARD"`) is
used as the label for all three strategy types.
