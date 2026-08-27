# Secret Sync Preflight

`sspf` is a read-only parity check for secret **key names**. It compares a desired manifest with key-only exports from CI and hosting destinations, catches missing, extra, likely-renamed, and over-limit states, and blocks dangerous delete plans before deployment. It never stores, fetches, or prints secret values.

Built for DevOps teams whose configuration crosses IaC, a secret manager, CI, and one or more hosting providers.

## Install

Build the single binary from source with Rust 1.85 or newer:

```sh
cargo install --path .
sspf --version
```

The factory publishes release binaries separately. This worker does not publish packages.

## Usage

Create `preflight.toml`:

```toml
version = 1

[[environments]]
name = "production"
desired = ["API_URL", "DATABASE_URL", "SESSION_KEY"]

[[environments.destinations]]
name = "hosting"
export = "exports/hosting.keys"
limit = 100
delete_policy = "block"
```

Create the provider export as **one key name per line**. Blank lines and lines starting with `#` are ignored:

```text
API_URL
DATABASE_URL
SESSION_KEY
```

Run the preflight:

```sh
sspf check --manifest preflight.toml
```

Useful output modes:

```sh
# Stable machine-readable result
sspf check --manifest preflight.toml --format json

# GitHub Actions workflow annotations
sspf check --manifest preflight.toml --format github

# Keep the detailed report on the local runner
sspf check --manifest preflight.toml --format json --report preflight-report.json

# Treat otherwise safe extras as deployment-blocking drift
sspf check --manifest preflight.toml --strict-extra
```

`delete_policy` controls how extra destination keys are classified:

- `block`: an extra key is a dangerous deletion plan and fails the check.
- `warn`: an extra key is reported but only fails with `--strict-extra`.
- `allow`: an extra key is informational unless `--strict-extra` is set.

Exit codes are stable: `0` means safe to deploy, `1` means drift or an unsafe plan blocks deployment, and `2` means the manifest/export could not be read or validated. Likely renames are suggestions only; `sspf` never mutates a provider.

### GitHub Actions

```yaml
- name: Secret parity preflight
  run: sspf check --manifest preflight.toml --format github --report preflight-report.json
```

The export files should be produced by a least-privilege, read-only metadata command and kept on the runner. Do not put values in an export: `sspf` rejects dotenv assignments, JSON, and whitespace-delimited records to prevent accidental value handling.

## Manifest reference

- `version`: must be `1`.
- `environments[].name`: unique display name.
- `environments[].desired`: unique key names; empty arrays are valid.
- `environments[].destinations[].name`: unique within its environment.
- `export`: path to a key-only text file, resolved relative to the manifest.
- `limit`: optional positive provider key limit. Desired or current count at/over the boundary is reported.
- `delete_policy`: `block` (default), `warn`, or `allow`.
- `ignore`: optional key names excluded from both desired and current comparisons.

Keys may contain ASCII letters, digits, `_`, `-`, `.`, `/`, and `:`. Output is local unless the caller explicitly sends it elsewhere. Because key names can still be sensitive, reports should be treated as build artifacts with restricted access.

## Development

```sh
npm install
npm test
npm run build       # Rust release binary + static site at dist/site/
npm run build:site  # static site only at dist/site/
cargo package       # verify the publishable Rust crate
```

The static documentation site includes an in-browser demo. It does not use analytics, network requests, cookies, or storage; demo input stays in the tab and is discarded on refresh.

## Repository map

- `src/` — Rust CLI and comparison engine.
- `tests/` — CLI integration and seeded fixture coverage.
- `site/` — dependency-light Vite documentation and local-only demo.
- `examples/` — safe key-only example manifest and exports.
- `.factory/` — opportunity brief, visual system, and build handoff.

## Security and scope

This tool is metadata-only. It does not connect to providers, accept provider tokens, rotate values, replace a secret manager, or automatically delete anything. If an export contains an assignment such as `KEY=value`, parsing stops before the value is retained or echoed, and the error identifies only the line number.

Report security issues through the repository’s private security reporting channel. Do not include secret values or sensitive key names in a public issue.

## License

MIT. See [LICENSE](LICENSE).
