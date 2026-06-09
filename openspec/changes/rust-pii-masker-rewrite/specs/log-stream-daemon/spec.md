## ADDED Requirements

### Requirement: State token for resumability
For each input file, the daemon MUST compute a 32-hex-char state token:
`hex(sha256(format!("{}{}", input_path.display(), birth_secs)))[..32]`
where `birth_secs` is seconds-since-UNIX_EPOCH from `metadata.created()`, falling back to
`metadata.modified()`, falling back to path-only token (disables rotation detection).

#### Scenario: Log rotation detection
- **WHEN** an input file is replaced (new inode / different birth time)
- **THEN** the state token changes and the daemon restarts processing from line 0

#### Scenario: Birth time unavailable fallback
- **WHEN** both `metadata.created()` and `metadata.modified()` fail
- **THEN** a warning is logged and the token is derived from the path string alone

### Requirement: `.linenumber` checkpoint file
The daemon MUST write the last-processed 1-based line number to `<tmp_dir>/<token>.linenumber`
after completing each batch. On startup it reads this file to determine the resume point.

#### Scenario: Resume after restart
- **WHEN** the daemon is restarted after having processed 100 lines
- **THEN** it reads `.linenumber` = 100 and starts processing from line 101

#### Scenario: At-least-once semantics
- **WHEN** the daemon crashes mid-batch
- **THEN** on restart it re-processes from the last committed checkpoint line number

#### Scenario: Missing checkpoint file
- **WHEN** no `.linenumber` file exists for an input
- **THEN** the daemon starts from line 0 (beginning of file)

#### Scenario: Corrupt checkpoint file
- **WHEN** `.linenumber` exists but contains non-numeric content
- **THEN** a warning is logged and processing starts from line 0

### Requirement: maxLinesInOneGo cap
- **WHEN** the input file has more unread lines than `maxLinesInOneGo`
- **THEN** the daemon processes at most `maxLinesInOneGo` lines per beat, then stops and
  writes the checkpoint before the next beat

### Requirement: Append-mode output
The output file MUST be opened with `create(true).append(true)`. New masked lines are
appended; existing output content is never overwritten.

### Requirement: Missing input file is non-fatal
- **WHEN** the configured input file does not exist at beat time
- **THEN** the error is logged and the daemon continues to the next job without exiting

### Requirement: No .running lock file
The daemon MUST NOT create or check a `.running` lock file. The old JS mechanism is removed.
Concurrent daemon instances against the same job are the operator's responsibility.

### Requirement: Beat interval
- **WHEN** `beatInterval` is set to N milliseconds in config
- **THEN** the daemon sleeps N ms between the end of one full-jobs sweep and the start of the next

### Requirement: Job configuration schema
Each entry in `DaemonConfig.jobs` MUST specify:
- `jobName`: string identifier used in log messages
- `files.input`: absolute or relative path to the input log file
- `files.output`: absolute or relative path to the masked output file

### Requirement: Log throughput reporting
- **WHEN** a beat processes one or more lines for a job
- **THEN** a log line at INFO level reports the job name, lines processed, and elapsed time
