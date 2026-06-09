## ADDED Requirements

### Requirement: RegexSet prescreen
The engine MUST compile all enabled recognizer patterns into a single `RegexSet` and use it to
identify candidate recognizers before running individual `find_iter` passes.

#### Scenario: Clean line skips all recognizers
- **WHEN** a line contains no text that matches any pattern in the RegexSet
- **THEN** `analyze()` returns an empty `Vec<Finding>` without invoking any per-recognizer `find_iter`

#### Scenario: Candidate filtering
- **WHEN** a line matches patterns for recognizers A and B but not C
- **THEN** only recognizers A and B run `find_iter`; recognizer C is skipped

### Requirement: Validator gate
Every span found by a recognizer's `find_iter` MUST pass its associated validator before it is
promoted to a `Finding`. Spans that fail validation are silently dropped (not masked, not reported).

#### Scenario: Invalid Luhn card
- **WHEN** a string matches the CREDIT_CARD pattern but fails the Luhn checksum
- **THEN** `analyze()` returns no CREDIT_CARD finding for that span

#### Scenario: Valid Luhn card
- **WHEN** a string matches the CREDIT_CARD pattern and passes the Luhn checksum
- **THEN** `analyze()` returns a CREDIT_CARD finding with the correct byte span

### Requirement: Overlap resolution
When two or more findings cover overlapping byte spans, `resolve_overlaps()` MUST retain at most
one finding per non-overlapping region, selecting by (score desc, length desc, entity asc).

#### Scenario: Higher score wins overlap
- **WHEN** two findings overlap and one has a higher score
- **THEN** only the higher-scoring finding is retained

#### Scenario: Non-overlapping findings both retained
- **WHEN** two findings cover disjoint byte spans
- **THEN** both findings are returned

### Requirement: Empty-input safety
- **WHEN** `analyze("")` or `mask("")` is called
- **THEN** it returns an empty findings list / `Cow::Borrowed("")` without panicking

### Requirement: Cow::Borrowed on no-PII input
- **WHEN** `mask(text)` finds no validated PII in `text`
- **THEN** it returns `Cow::Borrowed(text)` (zero allocation)

### Requirement: Unicode safety
- **WHEN** the input contains multi-byte UTF-8 characters
- **THEN** masking produces valid UTF-8 output and does not panic or corrupt character boundaries

### Requirement: EngineBuilder country selection
- **WHEN** `EngineBuilder::countries(["US", "IN"])` is called
- **THEN** the built Engine includes global recognizers plus US and IN recognizers

#### Scenario: Default is global-only
- **WHEN** `Engine::builder().build()` is called with no country selection
- **THEN** only global recognizers (CREDIT_CARD, IBAN_CODE, EMAIL_ADDRESS, IP_ADDRESS, MAC_ADDRESS,
  CRYPTO_BTC, PHONE_NUMBER) are active

### Requirement: Custom recognizer injection
- **WHEN** `EngineBuilder::add_recognizer(raw)` is called with a valid `RawRecognizer`
- **THEN** the custom recognizer participates in detection alongside catalog recognizers

### Requirement: Entity disable/enable overrides
- **WHEN** `EngineBuilder::disable_entities(["PHONE_NUMBER"])` is called
- **THEN** PHONE_NUMBER is not active even though it is a global recognizer
