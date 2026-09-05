# Check secret key drift before deployment

`sspf` compares expected secret key names with destination exports. It is for DevOps teams moving configuration through CI and hosting services.

It finds missing, extra, renamed, and over-limit keys. It rejects secret values without printing them.

## Install the CLI

Install from this repository with Cargo:

```sh
cargo install --git https://github.com/B-Divyesh/sf-secret-sync-preflight
sspf --version
```

The installed package provides one `sspf` executable and needs no running service.

## Try the bundled sample

```sh
sspf demo
```

The command creates a new temporary folder with the bundled key-name files. It prints that folder and runs the real check.

The sample contains drift, so the command exits with code `1`. Compare its files with [`examples/`](examples/).

Open the [browser sample](https://secret-sync-preflight.sociobot.in/demo/). The sample stays in the tab and clears on refresh.

Use **Reset demo** to restore the sample. Use **Start for real** to leave without changing browser data.

## Check keys in CI

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

The `desired` field contains the expected keys.

Create each destination export with one key name per line. Blank lines and lines starting with `#` are allowed.

```sh
sspf check --manifest preflight.toml
sspf check --manifest preflight.toml --json
sspf check --manifest preflight.toml --format github
sspf check --manifest preflight.toml --format json --report preflight-report.json
```

The terminal, JSON, and GitHub formats write to standard output. A report file is created only when you use `--report`.

`delete_policy` controls extra keys:

- `block` reports the extra key and fails the check.
- `warn` reports the extra key without failing.
- `allow` reports the extra key without failing.
- `--strict-extra` makes an extra key fail under every policy.

The process uses these exit codes:

- `0`: the check passed.
- `1`: drift blocked deployment.
- `2`: an input could not be read or checked.

## Use key-name files

The CLI accepts one key name per line. Names may use ASCII letters, digits, `_`, `-`, `.`, `/`, and `:`.

The CLI rejects `KEY=value`, JSON, lines containing spaces, unsupported characters, and duplicate names. Errors show the line number without printing a value.

## Privacy and scope

The installed CLI reads the local manifest and destination exports you provide. It makes no network calls and accepts no provider token.

It does not create, update, or delete provider secrets. The check leaves its input files unchanged.

Key names can be sensitive. Restrict access to saved reports.

The browser sample uses no cookies, local storage, session storage, or IndexedDB. It requests files only from the product site.

## Develop and verify

Prerequisites: a current Rust toolchain, a C compiler, Node.js, and npm.

```sh
npm ci
npm test
npm run build
cargo package
```

Run these commands before a release. The build places the static site in `dist/site/`.

Deploy `dist/site/` with its `staticwebapp.config.json`. The deployed configuration returns the designed 404 and sends the documented security headers.

Claim ledger: [`.factory/claims.json`](.factory/claims.json).

## License

Free software under the [MIT License](LICENSE).
