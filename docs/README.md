# pii-masker documentation

Banking-grade PII detection and masking engine in Rust. Detects and masks
checksum-validated structured identifiers — credit cards, IBANs, national IDs,
tax numbers — across 30+ countries. No machine learning, no NER, no free-text
entity extraction.

## Contents

- [Usage](usage.md) — CLI subcommands, daemon setup, systemd unit
- [Configuration](configuration.md) — `pii-masker.json` schema reference
- [Recognizers](recognizers.md) — full catalog table
- [Architecture](architecture.md) — engine design, RE2 constraints, overlap resolution
- [Adding Recognizers](adding-recognizers.md) — how to extend the catalog to more countries
- [Security](security.md) — banking-grade notes, limitations, caveats
- [Contributing](CONTRIBUTING.md) — dev setup, release process

## Scope limit

This library detects **structured, checksum-validated identifiers only**. It does
**not** detect free-text entities such as person names, physical addresses, dates,
or locations — those require NLP/NER, which is explicitly out of scope. See
[security.md](security.md) for details.
