# pii-masker

Banking-grade PII detection and masking in Rust: checksum-validated identifiers
(financial, contact, network, and national IDs across many countries), as a
library, a CLI, and a resumable log-stream daemon.

## Install

```sh
cargo install pii-masker
```

Library only (no CLI deps):

```sh
cargo add pii-masker --no-default-features
```

## Quick start

```sh
echo "card 4111 1111 1111 1111, ip 10.0.0.1" | pii-masker mask
# card **** **** **** 1111, ip [IP_ADDRESS]

pii-masker scan --json < input.log
pii-masker run --config pii-masker.json      # daemon over configured jobs
pii-masker list --all-countries              # supported recognizers
```

With country packs:

```sh
printf 'pay 4111 1111 1111 1111 from 10.0.0.1, pan ABCDE1234F, ssn 219-09-9999\n' \
  | pii-masker mask --country IN,US
# card **** **** **** 1111, ip [IP_ADDRESS], pan [IN_PAN], ssn [US_SSN]
```

See [docs/](docs/README.md) for configuration, recognizers, architecture, security, and contributing.

## License

GPL-3.0-only. See [LICENSE](LICENSE).
