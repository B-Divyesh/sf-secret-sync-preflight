# Verify secret key drift before deployment — independent QA round 2

## Verdict: PASS

**Finding count: 0. Untested claim count: 0.**

Verified 5 September 2026 against implementation candidate
`09dffe769eaa3a882de07a207346de1a2569258f`, documentation SHA
`d614005779ded6560ccd11ba32764da4d894d505`, and
`https://secret-sync-preflight.sociobot.in`.

The commits after the implementation candidate change only README, changelog,
and factory handoff text. A fresh production build matches the live site for
every HTML page, script, stylesheet, font, image, icon, service worker, sitemap,
and robots file checked.

No product code was changed during this verification.

## First screen

Fresh 1440×900 desktop and 390×844 phone contexts showed this before scrolling:

- Job: “Check secret key drift before deployment.”
- Audience: “For DevOps teams syncing configuration across CI and hosting services.”
- First action: “Try it with sample data.”
- Result of the action: “Opens a sample drift report in this browser.”
- Facts: no provider login, the demo works offline after the first visit, and the software is free under the MIT License.

The title names the job. The text uses plain terms and contains no mood heading
or marketing metaphor. All facts remain in the first phone viewport.

## Live product result

The first action opened `/demo/` with no account or provider login. The page
immediately showed one missing key, two extra keys, one likely rename, and a
four-key destination against a three-key maximum. The persistent label said
“Demo — sample data, nothing is saved” and exposed both Reset demo and Start
for real.

Edits changed the visible report. Reset restored the exact bundled sample.
Refresh cleared an unsaved edit. Start for real returned home. Preloaded
`real:sentinel` values in local and session storage survived entry, reset,
exit, and refresh. The demo created no cookie or IndexedDB database.

Every observed browser request was a same-origin GET. After the service worker
became ready, an offline reload returned 200 and the drift report remained
usable. Returning online restored the “Available offline” state. No normal
route produced a console error or page error.

Normal, invalid, boundary, and recovery paths passed:

- Matching keys produced a safe result. Exactly reaching the configured limit produced a warning, while exceeding it blocked deployment.
- `warn` and `allow` reported extra keys without blocking. `block` blocked them.
- Assignments and duplicate keys produced specific errors. The assignment sentinel never appeared in the result.
- Limits 0 and 10,001 were rejected with the documented 1–10,000 range. Correcting the value restored a passed result.
- Enter and Space operated the report button. Native selection, Tab order, reset, and route navigation worked from the keyboard.

## Claims

All 14 commands in `.factory/claims.json` ran exactly as declared from a clean
checkout. All passed. Each claim ID occurs once in the test sources.

| Claim | Exact command | Result |
| --- | --- | --- |
| `demo-command` | `cargo test --test claims claim_demo_command_uses_bundled_key_only_files` | PASS |
| `cli-recording` | `npm run test:claims -- --grep @claim:cli-recording` | PASS, desktop and phone |
| `drift-report` | `cargo test --test claims claim_check_finds_the_advertised_drift_states` | PASS |
| `no-secret-values` | `cargo test --test claims claim_values_are_rejected_without_echoing_them` | PASS |
| `key-file-rules` | `cargo test --test claims claim_key_file_rules_accept_and_reject_the_documented_inputs` | PASS |
| `delete-policy` | `cargo test --test claims claim_delete_policy_and_strict_mode_cover_every_policy` | PASS |
| `ci-exit-codes` | `cargo test --test claims claim_exit_codes_describe_pass_drift_and_input_error` | PASS |
| `report-formats` | `cargo test --test claims claim_terminal_json_github_and_opt_in_report_are_observable` | PASS |
| `cli-local-read-only` | `npm run test:consumer` | PASS |
| `demo-entry` | `npm run test:claims -- --grep @claim:demo-entry` | PASS, desktop and phone |
| `demo-browser-isolation` | `npm run test:claims -- --grep @claim:demo-browser-isolation` | PASS, desktop and phone |
| `demo-browser-network` | `npm run test:claims -- --grep @claim:demo-browser-network` | PASS, desktop and phone |
| `free-mit` | `cargo test --test claims claim_repository_carries_the_mit_license` | PASS |
| `site-deployment` | `npm run test:claims -- --grep @claim:site-deployment` | PASS, desktop and phone |

The live landing page, legal pages, demo, README, package metadata, and CLI help
were cross-checked against the ledger. No unlisted, false, incomplete, or
untested public claim was found.

## Installed CLI

The documented command `cargo install --git
https://github.com/B-Divyesh/sf-secret-sync-preflight` succeeded in a separate
consumer root and installed one executable. The packaged crate also installed
in a clean consumer environment.

The installed artifact passed `--help` and `--version`. `sspf demo` created a
new temporary folder and exited 1 with the advertised missing, extra, rename,
and limit findings. A clean fixture exactly at its limit exited 0. An invalid
assignment exited 2, named the line, and did not print the sentinel. A clean
run after that failure exited 0. Input hashes and the consumer work directory
were unchanged, and the socket-denial probe recorded no network call.

This product has no backend. Tenant isolation, backend restart persistence,
health endpoints, and 429/Retry-After behavior therefore do not apply.

## Accessibility, routes, privacy, and links

- `/`, `/demo/`, `/privacy/`, and `/terms/` return 200. The designed unknown route returns HTTP 404, has the shared shell, and offers Return home.
- Every tested route has its own title, description, canonical, Open Graph and Twitter metadata, one H1, one main landmark, ordered headings, a skip link, and the shared header and footer.
- Direct loads, normal links, Back, and Forward focus and announce the destination H1.
- Axe found zero violations on all five routes at desktop and phone widths. A repeated 200% text check found no overflow, hidden controls, or serious accessibility issue.
- Every visible interactive target measured at least 44×44 CSS pixels. Keyboard focus used a visible 3 px blue outline with sufficient contrast.
- Reduced motion removed the hero transform, reduced its duration to 0.01 ms, and disabled smooth scrolling.
- All links across all routes resolved. The privacy contact reached GitHub Security Advisories through its expected login page.
- CSP permits only same-origin resources and forbids framing. HSTS, no-referrer, nosniff, and restrictive permissions headers are live.
- The factory `verify-url.sh` passed with one title, `lang="en"`, one H1, a main landmark, alt text, labeled buttons, and no console errors.

The 404 request produces the browser's expected failed-resource console line.
That line records the deliberate HTTP 404 and is not a broken page or defect.

## Quality gates and performance

From the clean checkout:

- `npm ci`: PASS, 0 vulnerabilities.
- `npm test`: PASS — 3 Rust unit tests, 8 Rust claim tests, 10 CLI integration tests, 1 doctest, the packaged-consumer test, and 24 Playwright tests.
- `npm run build`: PASS; produced `target/release/sspf` and `dist/site/`.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `cargo +1.85.0 test --locked`: PASS after installing the documented minimum toolchain.
- `cargo package`: PASS; 125.1 KiB compressed.

The production output contains 3.11 KB of initial shell JavaScript, 4.89 KB of
demo JavaScript, 15.17 KB of CSS, 63.1 KB of fonts, and a 46.6 KB hero image.
Live mobile Lighthouse scored 100 for performance, accessibility, best
practices, and SEO. FCP was 1.1 seconds, LCP 1.2 seconds, CLS 0, and total
blocking time 10 ms, with no run warning.

## Earlier finding disposition

All findings below were checked against the live page, clean source, and
outcome tests.

| Finding | Current disposition |
| --- | --- |
| F-2-1 | Closed. The real `sspf demo` SVG and accessible transcript appear before How it works and match fresh CLI output line for line. |
| F-2-2 | Closed. Demo folder contents, all rejection channels, every output mode, report opt-in, full request history, and offline actions are asserted. |
| F-2-3 | Closed. The landing action itself is tested and opens the populated sample route. |
| F-2-4 | Closed. Drift tests and the consumer test compare input bytes or hashes before and after use. |
| F-2-5 | Closed. A fresh packaged consumer receives one executable and needs no service or socket. |
| F-2-6 | Closed. The vague build promise was replaced by direct Cargo install instructions, and the literal Git install succeeded. |
| F-2-7 | Closed. The stale README minimum was removed; package compatibility still passes on Rust 1.85.0. |
| F-2-8 | Closed. The stale prebuilt-download availability sentence was removed. |
| F-2-9 | Closed. Blank lines and comments are covered by `key-file-rules`. |
| F-2-10 | Closed. The complete block, warn, and allow matrix is claim-tested. |
| F-2-11 | Closed. Strict mode is claim-tested under all three policies. |
| F-2-12 | Closed. The packaged consumer uses only the supplied local manifest and export. |
| F-2-13 | Closed. Socket calls are blocked and logged; no attempt occurs. |
| F-2-14 | Closed. Input hashes remain unchanged, and no provider connection occurs. |
| F-2-15 | Closed. Single-name lines, comments, blanks, and invalid multi-token lines are tested. |
| F-2-16 | Closed. Assignment, JSON, and space-separated lines are all rejected without disclosure. |
| F-2-17 | Closed. Every documented character class and adjacent invalid cases are tested. |
| F-2-18 | Closed. The public wording is a build instruction; `npm run build` produced `dist/site/`. |
| F-2-19 | Closed. The production-server contract and live headers and 404 response all passed. |
| F-2-20 | Closed. Every route has full metadata and the shared shell; load, link, and history focus and announcements pass. |
| F-2-21 | Closed. CLI help and terminal output use expected and destination terminology. The serialized `desired` field remains documented only for file compatibility. |
| F-2-22 | Closed. The subjective exit-code fact was replaced by tested privacy, offline, and license facts. |
| F-2-23 | Closed. The README H1 now names the job. |
| F-2-24 | Closed. Browser-sample wording now says the sample stays in the tab and clears on refresh. |
| F-2-25 | Closed. Scope copy now says the CLI makes no network calls and accepts no provider token. |
| F-2-26 | Closed. Input wording now names assignments, JSON, and lines containing spaces. |
| F-2-27 | Closed. The page says Preparing offline demo until service-worker readiness, then Available offline. |
| F-2-28 | Closed. The error says the line must contain one key name and gives the correction. |
| F-2-29 | Closed. Invalid counts say Fix the key list. |
| F-2-30 | Closed. The invalid result heading says Fix the key list. |
| F-2-31 | Closed. The clean state says No drift. |
| F-2-32 | Closed. Clipboard failure gives one recovery instruction without repetition. |
| F-2-33 | Closed. The first screen gives provider-login, offline-demo, and MIT-license facts. |
| F-2-34 | Closed. Explicit desktop and phone measurements found no target below 44×44 CSS pixels. |
| F-2-35 | Closed. The real CLI preview, privacy and scope section, and product one-line footer are present. |

Review 1 BLOCKING 1–4 and MAJOR 1–3 are closed by the corresponding evidence
above. Review 1 MINOR 1 remains confirmed: all links resolve, and the original
parity-lattice visual identity matches `.factory/design.md`. The earlier
independent verification's zero-defect result was also reproduced on the final
implementation.

## Evidence and remaining factory step

Evidence is under `/work/.evidence/verification-2/`, including desktop and
phone screenshots, browser results, claim-command results, consumer results,
live/build hashes, link results, text-resize results, and Lighthouse JSON.

There are no known product gaps. Registry publication remains optional and is
owned by the factory; it was not attempted.
