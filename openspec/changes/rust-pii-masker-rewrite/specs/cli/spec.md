## ADDED Requirements

### Requirement: Feature-gated CLI
The CLI (clap, env_logger, anyhow deps) MUST be behind the `cli` Cargo feature.
`cargo build --no-default-features` MUST compile the library without pulling in any CLI dependencies.
The `[[bin]]` entry in Cargo.toml carries `required-features = ["cli"]`.

### Requirement: Global flags
- `--config <PATH>`: Load configuration from the given JSON file. If absent, try `pii-masker.json`
  in the current directory; if that is also absent, use built-in defaults.
- `-v / --verbose`: One flag = DEBUG level. CLI-level override of `runMode` from config.
- `RUST_LOG` env var is respected and overrides both `-v` and `runMode`.

### Requirement: `mask` subcommand
Reads from `--in <PATH>` or stdin (default stdin). Writes to `--out <PATH>` or stdout (default
stdout). Masks each line in order.

- `--country <CODES>`: comma-separated ISO-3166-1 alpha-2 codes to activate (e.g. `US,IN`)
- `--all-countries`: activate all country packs

#### Scenario: Stdin to stdout
- **WHEN** `echo "card 4111111111111111" | pii-masker mask`
- **THEN** stdout contains the masked line with card replaced, exit code 0

#### Scenario: Country activation
- **WHEN** `pii-masker mask --country IN` is called with a line containing a valid IN_PAN
- **THEN** the PAN is masked in output

#### Scenario: Invalid Luhn passes through
- **WHEN** a card-like number that fails the Luhn checksum is in the input
- **THEN** it appears unmodified in the output

### Requirement: `scan` subcommand
Reads from `--in <PATH>` or stdin. Prints findings.

- Default: human-readable table (entity, span start:end, score)
- `--json`: emit a JSON array `[{"entity","country","start","end","score"}]` to stdout
- `--country` / `--all-countries`: same as `mask`
- Exit code 0 even when no findings

#### Scenario: JSON output schema
- **WHEN** `echo "4111111111111111" | pii-masker scan --json`
- **THEN** stdout is a valid JSON array with one object containing entity="CREDIT_CARD"

### Requirement: `run` subcommand (daemon)
- Reads daemon job configuration from the loaded config file
- Executes one beat loop iteration per `beatInterval` milliseconds
- Each job: read input file from checkpoint, mask up to `maxLinesInOneGo` lines, append to
  output file, update `.linenumber` state file
- Runs until SIGTERM / Ctrl-C; each beat's errors are logged but do not terminate the loop

### Requirement: `list` subcommand
- Lists all recognizers from the catalog (filtered by country selection)
- Columns (human): entity, country, has-checksum, score, default mask strategy
- `--json`: emit a JSON array of recognizer metadata
- `--country` / `--all-countries`: filter / expand active set
- Exit code 0

### Requirement: Exit codes
- 0: success
- 1: user error (bad arguments, config parse failure, unknown subcommand)
- 2: runtime I/O error (file not found in mask/scan, unreadable config)
