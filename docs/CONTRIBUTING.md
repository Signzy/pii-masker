# Contributing

## Dev setup

```sh
git clone https://github.com/Signzy/pii-masker
cd pii-masker
cargo build --all-features
cargo test --all-features
```

Rust stable ≥ 1.74 required. Run `rustup update stable` to upgrade.

## Quality gates

Before opening a PR, run all local gates:

```sh
# format check
cargo fmt --all --check

# lint (CI enforces -D warnings)
cargo clippy --all-targets --all-features -- -D warnings

# tests (including catalog self-test and proptest)
cargo test --all-features

# lib compiles without CLI deps
cargo build --no-default-features

# bench (optional, documents throughput)
cargo bench
```

Or use the ci-check script if present:
```sh
./ci-check.sh
```

## Adding a recognizer

See [adding-recognizers.md](adding-recognizers.md) for the full per-entry contract.
Key points:
- ASCII classes only (`[0-9]`, not `\d`)
- ≥ 3 examples and ≥ 3 counter_examples
- `validator = "none"` + reduced score if checksum unverified
- Source citation in TOML comment

Run `cargo test --test catalog` to verify vectors before submitting.

## Release process

1. Update `version` in `Cargo.toml` (semantic versioning).
2. Add an entry to `CHANGELOG.md` documenting the change.
3. Commit: `git commit -m "chore: release vX.Y.Z"`.
4. Tag: `git tag vX.Y.Z && git push --tags`.
5. The `release.yml` workflow runs `cargo publish --dry-run` then publishes to
   crates.io using the `CARGO_REGISTRY_TOKEN` secret.

### Setting up `CARGO_REGISTRY_TOKEN`

1. Log in to [crates.io](https://crates.io) with the Signzy account.
2. Go to Account Settings → API Tokens → New Token.
3. Add the token as a repository secret named `CARGO_REGISTRY_TOKEN` in GitHub.

### Before first publish

Confirm the crate name `pii-masker` is available on crates.io:
```sh
cargo publish --dry-run
```
If the name is taken, update `name` in `Cargo.toml`, the `[[bin]].name`, and the
`docs/` references to use the fallback name `pii-sanitizer`.

## Commit style

Conventional commits are preferred:
- `feat: add XY_ENTITY recognizer`
- `fix: correct Verhoeff table for Aadhaar`
- `chore: bump version to 0.2.0`
- `docs: update configuration schema`
- `test: add catalog vectors for CN_RIC`
