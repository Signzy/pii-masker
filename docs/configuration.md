# Configuration

`pii-masker` loads configuration from `pii-masker.json` (in the current directory by
default) or from a path given to `--config`. If no file is found, built-in defaults apply.

## Schema

```jsonc
{
  "engine": {
    // Country packs to activate. Default: global only.
    // String "all" activates every pack; array activates specific packs.
    "countries": "all",           // or ["US", "IN", "DE"]

    // Entity types to suppress (even if their country pack is active).
    "disableEntities": ["PHONE_NUMBER"],

    // Entity types to force-enable (override country opt-in).
    "enableEntities": [],

    // Per-entity mask strategy overrides.
    "maskOverrides": {
      "EMAIL_ADDRESS": { "kind": "hash" },
      "CREDIT_CARD":   { "kind": "partial", "suffix": 4, "mask_char": "*" }
    },

    // Additional custom recognizers not in the built-in catalog.
    "customRecognizers": [
      {
        "entity": "MY_INTERNAL_ID",
        "country": "",
        "regex": "\\bMY-[0-9]{6}\\b",
        "validator": "none",
        "score": 0.9,
        "mask": { "kind": "redact" },
        "examples": ["MY-123456", "MY-000001", "MY-999999"],
        "counter_examples": ["MY-12345", "MY-1234567", "hello"]
      }
    ]
  },

  "daemon": {
    // Milliseconds between beats. Default: 5000.
    "beatInterval": 5000,

    // Directory for .linenumber checkpoint files. Default: /tmp.
    "tmpDirectory": "/tmp",

    // Maximum lines to process per job per beat. Default: 5000.
    "maxLinesInOneGo": 5000,

    // Logging level: "debug"/"verbose", "error", "noLog", or omit for "info".
    "runMode": "info",

    // Jobs to process each beat.
    "jobs": [
      {
        "jobName": "nginx-access",
        "files": {
          "input":  "/var/log/nginx/access.log",
          "output": "/var/log/nginx/access.masked.log"
        }
      }
    ]
  }
}
```

## Country codes

Use [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) codes:
`US`, `UK`, `IN`, `DE`, `FR`, `BR`, `CN`, etc. See `pii-masker list --all-countries`
for the full list of active recognizers.

**Default**: country packs are **opt-in**. Global recognizers (CREDIT_CARD, IBAN_CODE,
EMAIL_ADDRESS, IP_ADDRESS, MAC_ADDRESS, CRYPTO_BTC, PHONE_NUMBER, SWIFT_BIC, CRYPTO_ETH)
are always active.

## Mask strategy overrides

| Field | Values | Default |
|---|---|---|
| `kind` | `"partial"`, `"redact"`, `"hash"` | per-entity default |
| `prefix` | integer ≥ 0 | 0 |
| `suffix` | integer ≥ 0 | 0 |
| `mask_char` | single character string | `"*"` |
| `label` | string | entity name |

- `redact` → `[LABEL]`
- `partial` → keeps first `prefix` and last `suffix` alnum chars visible
- `hash` → `[LABEL:aabbccddee11]` (SHA-256 first 12 hex chars, deterministic)
