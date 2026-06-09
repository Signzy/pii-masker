# Usage

## Installation

```sh
# Install the CLI binary
cargo install pii-masker

# Add as a library (no CLI deps)
cargo add pii-masker --no-default-features

# Add as a library with phone feature (strict phone validation)
cargo add pii-masker --no-default-features --features phone
```

## CLI subcommands

### `mask` — stream masking

Reads line-by-line from stdin or a file, writes masked output to stdout or a file.

```sh
# stdin → stdout (global recognizers only)
echo "card 4111 1111 1111 1111, ip 10.0.0.1" | pii-masker mask

# file → file
pii-masker mask --input access.log --output masked.log

# activate US + India country packs
cat app.log | pii-masker mask --country US,IN

# activate every country pack
cat app.log | pii-masker mask --all-countries
```

### `scan` — findings report

Prints detected entities without modifying input.

```sh
# human-readable table
pii-masker scan < input.log

# JSON array of findings
pii-masker scan --json < input.log
# [{\"entity\":\"CREDIT_CARD\",\"country\":null,\"start\":5,\"end\":21,\"score\":0.85}]
```

### `run` — daemon

Reads jobs from the config file, processes each input log up to `maxLinesInOneGo`
lines per beat, and appends masked output to the configured output file.

```sh
pii-masker run --config pii-masker.json

# process once and exit (no loop)
pii-masker run --config pii-masker.json --once
```

#### systemd unit (example)

```ini
[Unit]
Description=PII Masker daemon
After=network.target

[Service]
ExecStart=/usr/local/bin/pii-masker run --config /etc/pii-masker.json
Restart=on-failure
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

### `list` — catalog browser

```sh
# list global recognizers
pii-masker list

# list all recognizers (all packs)
pii-masker list --all-countries

# JSON output (pipe to jq, etc.)
pii-masker list --all-countries --json | jq '.[] | .entity'
```

## Verbosity and logging

```sh
# -v sets DEBUG level
pii-masker -v mask < input.log

# RUST_LOG overrides -v and runMode
RUST_LOG=warn pii-masker run --config cfg.json
RUST_LOG=pii_masker=debug pii-masker mask < input.log
```

## Library usage

```rust
use pii_masker::{Engine, EngineBuilder};

fn main() -> pii_masker::Result<()> {
    // Global recognizers only (credit cards, IBAN, email, IP, MAC, phone)
    let engine = Engine::builder().build()?;
    let masked = engine.mask("card 4111 1111 1111 1111, ip 10.0.0.1");
    println!("{masked}");

    // With country packs
    let engine = EngineBuilder::default()
        .countries(["US", "IN"])
        .build()?;
    let masked = engine.mask("ssn 219-09-9999, pan ABCDE1234F");
    println!("{masked}");
    Ok(())
}
```
