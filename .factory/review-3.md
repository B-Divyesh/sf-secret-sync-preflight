# Check secret key drift before deployment — review 3

## Verdict: PASS

**Finding count: 0. Untested claim count: 0.**

Reviewed 5 September 2026 against implementation candidate
`09dffe769eaa3a882de07a207346de1a2569258f`, documentation SHA
`9f56607944ddb8cee8a2d8cb9e18289f6479774c`, and the live site at
`https://secret-sync-preflight.sociobot.in`.

The commits after the implementation candidate change only README, changelog,
handoff, and verification text. The current production build and live
`index.html` have the same SHA-256:
`2c4cee28a0149a8a6aa06add07a24500ccaa271b9d194f0d3821afc0d7d5d9ae`.
No product code was changed for this review.

## First screen

Fresh 1440×900 desktop and 390×844 phone browser contexts showed this before
scrolling:

- Job: “Check secret key drift before deployment.”
- Audience: “For DevOps teams syncing configuration across CI and hosting services.”
- First action: “Try it with sample data.”
- Action result: “Opens a sample drift report in this browser.”
- Facts: no provider login, offline use after the first visit, and free under
  the MIT License.

The live title names the job. The first screen uses plain, task-specific words;
it has no metaphor or mood heading. The phone context kept all three facts in
the initial viewport.

## Live browser checks

The first action opened `/demo/` without a login. It immediately showed one
missing key, two extra keys, one likely rename, and four keys against a maximum
of three. The persistent label read “Demo — sample data, nothing is saved.”

Reset restored the bundled expected-key list. A `real:review3` local- and
session-storage sentinel remained unchanged after entering the demo, editing,
resetting, leaving, and reloading. The demo used no cookie, local storage,
session storage, or IndexedDB data of its own.

After service-worker readiness, a live phone context reloaded the demo offline
with HTTP 200, showed “Offline · demo still works”, and produced the unsafe
report. The complete observed flow made only same-origin GET requests. No page
or console error occurred.

Desktop and phone Axe scans found zero violations on the landing page and
demo. Fresh desktop scans also found zero violations on `/`, `/demo/`,
`/privacy/`, `/terms/`, and the designed unknown route. Each has one `h1` and
one `main`; the unknown route deliberately returns HTTP 404 and offers Return
home. Route titles were correct on all five routes.

The local browser suite also verified normal, invalid, boundary, warning,
recovery, keyboard, visible focus, target size, reduced-motion, route history,
link, metadata, and console behavior at desktop and phone sizes.

## Claims and clean consumer check

Ran `npm ci` and every command in `.factory/claims.json` from a fresh clone.
Each command passed.

| Claim | Command | Result |
| --- | --- | --- |
| `demo-command` | `cargo test --test claims claim_demo_command_uses_bundled_key_only_files` | PASS |
| `cli-recording` | `npm run test:claims -- --grep @claim:cli-recording` | PASS |
| `drift-report` | `cargo test --test claims claim_check_finds_the_advertised_drift_states` | PASS |
| `no-secret-values` | `cargo test --test claims claim_values_are_rejected_without_echoing_them` | PASS |
| `key-file-rules` | `cargo test --test claims claim_key_file_rules_accept_and_reject_the_documented_inputs` | PASS |
| `delete-policy` | `cargo test --test claims claim_delete_policy_and_strict_mode_cover_every_policy` | PASS |
| `ci-exit-codes` | `cargo test --test claims claim_exit_codes_describe_pass_drift_and_input_error` | PASS |
| `report-formats` | `cargo test --test claims claim_terminal_json_github_and_opt_in_report_are_observable` | PASS |
| `cli-local-read-only` | `npm run test:consumer` | PASS |
| `demo-entry` | `npm run test:claims -- --grep @claim:demo-entry` | PASS |
| `demo-browser-isolation` | `npm run test:claims -- --grep @claim:demo-browser-isolation` | PASS |
| `demo-browser-network` | `npm run test:claims -- --grep @claim:demo-browser-network` | PASS |
| `free-mit` | `cargo test --test claims claim_repository_carries_the_mit_license` | PASS |
| `site-deployment` | `npm run test:claims -- --grep @claim:site-deployment` | PASS |

The installed-artifact test packaged the crate, installed it into a separate
consumer root, confirmed one `sspf` executable, denied and logged socket calls,
provided a decoy provider token, and compared input hashes. It passed with no
network attempt, no input change, no running service, and no implicit report.

The claim ledger, live landing copy, demo, README, privacy page, terms page,
and CLI documentation were cross-checked. No public promise is missing from
the ledger, false, incomplete, or untested.

## Quality gates and privacy

The following passed from the clean checkout:

```sh
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
```

`npm test` covered Rust unit, claim, CLI integration, doctest, installed
consumer, and 24 desktop/phone browser tests. `npm run build` produced the
release CLI and `dist/site/`. Packaging verified the distributable crate.

Live `/` returned HTTP 200 and the configured CSP, `Referrer-Policy:
no-referrer`, `X-Content-Type-Options: nosniff`, Permissions Policy, and HSTS.
The designed unknown route returned HTTP 404 with the same relevant headers.
The product has no backend, so tenant isolation, restart persistence, health
endpoints, and rate-limit behavior do not apply.

## Earlier finding disposition

All findings in review 1 and F-2-1 through F-2-35 in review 2 remain closed.
This review repeated their public outcomes: clear first screen, real CLI
recording, full claim coverage, route shell and metadata, consistent terms,
plain copy, demo isolation and recovery, privacy and offline behavior, 44 px
targets, and the required landing-page sections. The prior verification’s zero
finding result is confirmed, with no reopened issue.

## Evidence

Review evidence is in `/work/.evidence/review-3/`, including desktop and phone
screenshots, live browser results, and the privacy/offline request record.
