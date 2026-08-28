# Secret Sync Preflight

`sspf` checks expected secret key names against destination key exports before deployment. It is for DevOps teams moving configuration through CI and hosting services.

It finds missing, extra, renamed, and over-limit keys. It rejects secret values without printing them. See [the claim ledger](.factory/claims.json) for runnable proof.

## Install

Rust 1.85 or newer is required.

```sh
cargo install --path .
sspf --version
```

Prebuilt downloads are not available yet.

## Try the bundled demo

```sh
sspf demo
```

The command writes key-name-only sample files to a new temporary directory, prints that directory, and runs the real check. It intentionally returns `1` because the sample contains drift.

Open [the browser sample](https://secret-sync-preflight.sociobot.in/?demo=1). It uses an isolated page-memory sample. Use **Reset demo** to restore it, or **Start for real** to leave it.

## Use in CI

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

Create an export with one key name per line. Blank lines and `#` comments are allowed.

```sh
sspf check --manifest preflight.toml
sspf check --manifest preflight.toml --format json
sspf check --manifest preflight.toml --format github
sspf check --manifest preflight.toml --format json --report preflight-report.json
```

`delete_policy` controls extra keys: `block` fails, `warn` reports, and `allow` reports. `--strict-extra` makes extras fail in every policy.

Exit code `0` means pass. Exit code `1` means drift. Exit code `2` means invalid input.

## Privacy and scope

The CLI reads local key-name files. It has no telemetry or provider integration. It does not create, update, or delete provider secrets. Keep local reports restricted because key names can be sensitive.

Input must contain one key name per line. A line such as `KEY=value`, JSON, or a whitespace-delimited record is rejected. Valid key characters are ASCII letters, digits, `_`, `-`, `.`, `/`, and `:`.

## Develop and verify

```sh
npm install
npm test
npm run build
cargo package
```

The static site builds to `dist/site/`. Deploy it as a static site; `staticwebapp.config.json` supplies headers and the designed 404 response.

## License

MIT. See [LICENSE](LICENSE).
