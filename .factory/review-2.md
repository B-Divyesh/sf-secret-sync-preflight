# Adversarial first-read review 2 — Secret Sync Preflight

## Verdict: FAIL

Reviewed 2026-08-28 UTC at `https://secret-sync-preflight.sociobot.in` in fresh Chromium contexts at 390×844 and 1440×900, and against repository commit `3bc558f0ef5b24450a1938f9c16ee837997ee623`.

The first screen is now clear and the browser demo works. The review still has blocking findings: the required real-CLI recording is absent, public claims remain unlisted or only partly tested, and three earlier findings were only partly repaired. PASS requires zero findings and no untested claim.

## Cold first read, before scrolling

On both viewports I could answer all three questions:

- What it does: checks whether expected secret key names differ from keys in CI or hosting destinations before deployment.
- For whom: DevOps teams syncing configuration through CI and hosting services.
- What to click first: **Try it with sample data**; the adjacent text says it opens a seeded report in the browser.

The exact first-screen text was “Check secret key drift before deployment.”, “For DevOps teams syncing configuration across CI and hosting services.”, and “Try it with sample data”. The mobile first screen showed the action and all three fact lines without scrolling. This closes review-1 **BLOCKING 1**.

## Findings, ordered by severity

### F-2-1 — BLOCKING — the CLI still has no landing-page recording (reopens review-1 BLOCKING 2)

**Quote/location:** landing install terminal: `cargo install --git …`, `sspf check --manifest …`; repository search finds no recording, asciinema asset, or SVG recording of `sspf demo`.

The browser sample, banner, Reset, Start for real, `/demo/`, `.factory/demo.md`, bundled examples, and `sspf demo` now work. However, the CLI demo contract also requires a self-hosted recording of the real binary doing the sample job. The existing terminal is a static install snippet and never shows `sspf demo`. A visitor is still trying a browser reimplementation rather than seeing the shipped CLI execute.

**Concrete fix:** record the actual `sspf demo` command against the bundled fixture, publish the self-hosted recording on the landing page before “How it works”, caption its exit status and temporary output path, and test that the rendered recording contains the current real CLI output.

### F-2-2 — BLOCKING — passing claim commands do not test their full listed promises (reopens review-1 BLOCKING 3)

**Quote/location:** `.factory/claims.json`, `tests/claims.rs`, and `e2e/site.spec.ts`.

- `report-formats` promises terminal, JSON, and GitHub output plus no report file unless requested. Its test invokes only `--format github` with `--report`; it never invokes terminal or JSON stdout and never checks the absence of a file without `--report`.
- `no-secret-values` checks only that the sentinel is absent from stderr. It does not assert stdout, report files, or all rejected input forms named in the README.
- `demo-browser-network` checks the collected origins immediately after the first navigation. It does not reassert the request log after service-worker setup, reload, offline use, and the demo action.
- `demo-command` asserts printed phrases and exit 1, but does not parse the printed directory, confirm that it is new, or compare its files with the bundled sample.

These are untested parts of listed claims even though every command exits successfully.

**Concrete fix:** extend the one tagged test for each claim to assert every clause above. Keep one `@claim:<id>` test per entry, but make that test exercise all promised modes and side effects.

### Unlisted public claims — BLOCKING (also reopens review-1 BLOCKING 3)

Each row is a public sentence a visitor can rely on but which has no matching `.factory/claims.json` entry and clean-sandbox test.

| ID | Exact quote/location | Why this is unverified | Concrete fix |
| --- | --- | --- | --- |
| F-2-3 | Landing: “Opens a seeded drift report in this browser.” | The tagged browser tests start directly at `/demo/`; no claim test starts at the landing action. | Add `demo-entry` and click the landing action before asserting the seeded metrics and URL. Replace “seeded” with “sample” in the copy. |
| F-2-4 | Landing: “The CLI compares key names and makes no changes.” | Drift output is tested, but input immutability is not. | Add a claim that hashes every manifest/export before and after the CLI run. |
| F-2-5 | Landing: “Runs locally as one CLI.” | No claim test proves the packaged result is one runnable binary with no runtime service. | Add a packaged-binary test in a fresh directory with outbound network denied. |
| F-2-6 | Landing: “Build from source today.” | The build gate is not registered as a public claim test. | Register a clean source-build claim, or change the section to a direct factual install instruction without this promise. |
| F-2-7 | README: “Rust 1.85 or newer is required.” | Current tests use the installed compiler, not the stated minimum. | Add a CI/claim job pinned to Rust 1.85. |
| F-2-8 | Landing and README: “Prebuilt downloads are not available yet.” | Availability is not checked and can become stale. | Add a release-link/availability test or replace this with a maintained release link when binaries exist. |
| F-2-9 | README: “Blank lines and `#` comments are allowed.” | No claim entry covers either input case. | Add a fixture containing both and assert the same report as its stripped form. |
| F-2-10 | README: “`delete_policy` controls extra keys: `block` fails, `warn` reports, and `allow` reports.” | An untagged test covers `warn`; `allow` and the complete three-policy matrix are not claim-tested. | Add one tagged policy-matrix claim test. |
| F-2-11 | README: “`--strict-extra` makes extras fail in every policy.” | Only `warn` is exercised with strict mode. | Extend the policy claim test to `block`, `warn`, and `allow`. |
| F-2-12 | README: “The CLI reads local key-name files.” | The ledger does not delimit or trace CLI reads. | Add a filesystem-observation test using only a temporary manifest and exports. |
| F-2-13 | README: “It has no telemetry or provider integration.” | The network claim covers the browser only. | Add a CLI no-network claim and run the binary with networking denied or traced. |
| F-2-14 | README: “It does not create, update, or delete provider secrets.” | No test checks fixture hashes or attempted provider/network activity. | Add a no-mutation claim with before/after hashes and denied network. |
| F-2-15 | README: “Input must contain one key name per line.” | The value-rejection claim tests one assignment only. | Add valid single-line and invalid multi-record cases to a tagged input-format claim. |
| F-2-16 | README: “A line such as `KEY=value`, JSON, or a whitespace-delimited record is rejected.” | Only `KEY=value` is claim-tested. | Test all three named input classes without echoing either key or value. |
| F-2-17 | README: “Valid key characters are ASCII letters, digits, `_`, `-`, `.`, `/`, and `:`.” | No claim test exercises every allowed class and adjacent rejected characters. | Add a table-driven input-character claim test. |
| F-2-18 | README: “The static site builds to `dist/site/`.” | `npm run build` proves this during this review, but the promise is absent from the ledger. | Register the build-output assertion or remove the output-path claim. |
| F-2-19 | README: “`staticwebapp.config.json` supplies headers and the designed 404 response.” | The local server checks the 404 body/status, not deployed header behavior from that config. | Add a deployed-config contract test for the headers and 404 override, or describe the file without promising its effect. |

### F-2-20 — BLOCKING — route focus and route metadata remain incomplete (reopens review-1 MAJOR 1)

**Quote/location:** clicking header “Demo” changes `/` to `/demo/`, but `document.activeElement` is `BODY`; Back returns to `/` with focus still on `BODY`. `legal.ts` focuses an H1 only when the query string contains `from`.

The special “Start for real” path now focuses the landing H1, but ordinary links, deep-link loads, and Back do not move focus to or announce the new H1. No route-change `aria-live="polite"` announcement exists. Metadata is also partial: Demo, Privacy, and Terms have only `twitter:card`, without Twitter title, description, or image. The 404 has no canonical, Open Graph, Twitter, or apple-touch metadata. Its header omits “How it works”, and its footer omits the wordmark, product one-liner, and Source link.

**Concrete fix:** use one shared shell/head template for all routes; include the full metadata set on each route; focus and announce the destination H1 for every in-site route transition and Back/Forward restoration; test the normal header links, not only `?from=demo`.

### F-2-21 — BLOCKING — user-facing terminology still changes (reopens review-1 MAJOR 2)

**Quote/location:** the site says “expected keys” and “destination keys”; `sspf check --help` says “desired key manifest”; terminal output says “3 desired” and “4 current”; the README schema uses `desired`.

The website terminology was repaired, but the CLI—the product users actually run—still changes terms. “Current” does not say current where, and “desired” conflicts with the learned “expected” term.

**Concrete fix:** keep the serialized compatibility field `desired` if required, but label it as “expected keys” in docs and use “expected” / “destination” in CLI help and human output.

### Copy findings — BLOCKING because review-1 MAJOR 3 remains only partly repaired

All landing and README sentences are now at or below 22 words, and all action buttons name a result. The following earlier clarity defects remain on the live site or in the README.

| ID | Exact quote/location | Why it fails first-read clarity | Proposed rewrite |
| --- | --- | --- | --- |
| F-2-22 | Landing fact: “Uses clear CI exit codes” | “Clear” is subjective; the useful fact is the actual mapping. | “Uses exit codes 0, 1, and 2” |
| F-2-23 | README H1: “Secret Sync Preflight” | A product name alone does not describe the document out of context. | “Check secret key drift before deployment” |
| F-2-24 | README: “It uses an isolated page-memory sample.” | “Page-memory” is implementation jargon. | “The sample stays in this tab and clears on refresh.” |
| F-2-25 | README: “It has no telemetry or provider integration.” | “Provider integration” is vague. | “It sends no telemetry and never connects to a secret provider.” |
| F-2-26 | README: “A line such as `KEY=value`, JSON, or a whitespace-delimited record is rejected.” | “Whitespace-delimited record” is avoidable jargon. | “The CLI rejects `KEY=value`, JSON, and lines containing spaces.” |
| F-2-27 | Demo status: “Ready offline” | It appears before service-worker readiness is confirmed and does not explain when offline use becomes available. | Show “Available offline” only after `navigator.serviceWorker.ready`; otherwise show “Preparing offline demo”. |
| F-2-28 | Demo error: “Line 1 is not a key-only record.” | “Key-only record” is jargon. | “Line 1 must contain one key name.” |
| F-2-29 | Demo count state: “Check input” | It does not say what to fix. | “Fix the key list” |
| F-2-30 | Demo result heading: “Input needs attention” | It is vague and duplicates the specific error below. | “Fix the key list” |
| F-2-31 | Demo clean finding: “Aligned” | It introduces a new synonym for no drift/pass. | “No drift” |
| F-2-32 | Clipboard fallback: “Clipboard access was unavailable. Select the command above to copy it.” | The first sentence repeats the “Copy unavailable” button state. | “Select the command above to copy it.” |

### F-2-33 — MAJOR — the first-screen facts are not privacy, offline, and price facts

**Quote/location:** “Rejects secret values” / “Finds key drift” / “Uses clear CI exit codes”.

These repeat capability copy. The mandatory first-screen shape asks for the three practical facts a visitor needs before trying a security tool: privacy, offline behavior, and price.

**Concrete fix:** after registering exact claim tests, use “No provider login”, “Works offline after the first visit”, and “Free and open source”.

### F-2-34 — MAJOR — three repeated navigation targets are smaller than 44×44 px

**Quote/location:** measured in Chromium at 390 px: header “Demo” is 40×44 px, header “Install” is 85×40 px, and footer “Terms” is 40×44 px. The same width/height failures occur on desktop.

These targets miss the stated 44 px minimum in one dimension. Axe does not report target-size failures, so the existing serious/critical-only assertion does not catch this.

**Concrete fix:** give every header/footer link a minimum inline size of 44 px and restore the Install link to at least 44 px high. Add explicit bounding-box assertions at 390 px.

### F-2-35 — MAJOR — the landing skeleton omits a privacy/scope section and a product one-line footer

**Quote/location:** the landing order moves from “How it works” to a demo link and Install. The footer says “Key names can be sensitive. Store reports on your CI runner.”

The required “What it does not do / privacy” section is absent from the landing page. The footer contains a warning but no product one-liner. The required product/live preview before “How it works” is also absent; F-2-1’s real CLI recording would fill that gap.

**Concrete fix:** place the real CLI recording immediately after the hero, add a short scope section that names local files, no provider mutation, and report sensitivity, and add the product one-liner to the footer.

## Copy audit

Counts treat hyphenated compounds and inline-code tokens as one word and ignore punctuation-only marks. Code samples and sample key values are not prose sentences. Repeated identical actions are listed once with their count.

### Live landing page

| # | Words | Location | Exact copy | Result |
| ---: | ---: | --- | --- | --- |
| 1 | 7 | Title | Secret Sync Preflight — check secret key drift | OK |
| 2 | 12 | Meta description | A CLI for DevOps teams to find secret key drift before deployment. | OK |
| 3 | 3 | Nav | Skip to content | OK |
| 4 | 3 | Nav | How it works | OK |
| 5 | 1 | Nav | Demo | OK; link, not an action button |
| 6 | 1 | Nav | Privacy | OK; link |
| 7 | 1 | Nav | Install | OK; link |
| 8 | 3 | Hero label | Key names only | OK |
| 9 | 6 | H1 | Check secret key drift before deployment. | OK |
| 10 | 10 | Hero | For DevOps teams syncing configuration across CI and hosting services. | OK |
| 11 | 5 | Primary action ×2 | Try it with sample data | OK |
| 12 | 8 | Action note | Opens a seeded drift report in this browser. | F-2-3: “seeded” is jargon and the path lacks a claim entry |
| 13 | 3 | Secondary action | Install the CLI | OK |
| 14 | 3 | Fact | Rejects secret values | OK; listed claim |
| 15 | 3 | Fact | Finds key drift | OK; listed claim |
| 16 | 5 | Fact | Uses clear CI exit codes | F-2-22 |
| 17 | 3 | Figure caption | Four configuration layers. | OK |
| 18 | 4 | Figure caption | One expected key list. | OK |
| 19 | 21 | Image alt | Four dark geometric configuration layers with aligned key-shaped cells, one missing cell, one displaced coral cell, and an amber capacity boundary. | OK |
| 20 | 2 | Check label | Missing keys | OK |
| 21 | 2 | Check label | Extra keys | OK |
| 22 | 2 | Check label | Likely renames | OK |
| 23 | 2 | Check label | Destination limits | OK |
| 24 | 2 | Check label | Delete policy | OK |
| 25 | 2 | Section label | Key-name checks | OK |
| 26 | 5 | H2 | Compare expected and destination keys. | OK |
| 27 | 5 | Body | Your repository lists expected keys. | OK |
| 28 | 7 | Body | Each destination export lists keys that exist. | OK |
| 29 | 9 | Body | The CLI compares key names and makes no changes. | F-2-4: unlisted claim |
| 30 | 3 | H3 | List expected keys | OK |
| 31 | 10 | Body | List each environment’s expected keys in a TOML file. | OK |
| 32 | 3 | H3 | Export destination keys | OK |
| 33 | 11 | Body | Export one key name per line with a read-only provider command. | OK for the named DevOps audience |
| 34 | 3 | H3 | Block unsafe deployment | OK |
| 35 | 7 | Body | Read a terminal, JSON, or GitHub report. | OK; listed claim, but see F-2-2 |
| 36 | 6 | Body | Unsafe results exit with an error. | OK; listed claim |
| 37 | 2 | Section label | Sample check | OK |
| 38 | 5 | H2 | See a sample drift report. | OK |
| 39 | 14 | Body | Open the isolated sample to edit key names and reset it at any time. | OK; listed isolation claim |
| 40 | 5 | Section label | Add the check to CI | OK |
| 41 | 5 | H2 | Runs locally as one CLI. | F-2-5: unlisted claim |
| 42 | 4 | Body | Build from source today. | F-2-6: unlisted claim |
| 43 | 6 | Body | Prebuilt downloads are not available yet. | F-2-8: unlisted availability claim |
| 44 | 3 | Button | Copy install command | OK; result-naming verb |
| 45 | 5 | Footer | Key names can be sensitive. | OK |
| 46 | 6 | Footer | Store reports on your CI runner. | OK |
| 47 | 5 | Footer | Built by Param Factory · v0.1.0 | OK |
| 48 | 3 | External link | Source (opens GitHub) | OK |

### README

| # | Words | Location | Exact copy | Result |
| ---: | ---: | --- | --- | --- |
| 1 | 3 | H1 | Secret Sync Preflight | F-2-23 |
| 2 | 12 | Body | `sspf` checks expected secret key names against destination key exports before deployment. | OK; listed drift claim |
| 3 | 12 | Body | It is for DevOps teams moving configuration through CI and hosting services. | OK |
| 4 | 8 | Body | It finds missing, extra, renamed, and over-limit keys. | OK; listed claim |
| 5 | 7 | Body | It rejects secret values without printing them. | OK; listed claim, but see F-2-2 |
| 6 | 7 | Body | See the claim ledger for runnable proof. | F-2-2: the ledger does not yet prove every clause |
| 7 | 1 | H2 | Install | OK |
| 8 | 6 | Body | Rust 1.85 or newer is required. | F-2-7: unlisted quantitative claim |
| 9 | 6 | Body | Prebuilt downloads are not available yet. | F-2-8 |
| 10 | 4 | H2 | Try the bundled demo | OK |
| 11 | 19 | Body | The command writes key-name-only sample files to a new temporary directory, prints that directory, and runs the real check. | Listed, but only partly asserted; F-2-2 |
| 12 | 9 | Body | It intentionally returns `1` because the sample contains drift. | OK; listed demo claim |
| 13 | 4 | Body | Open the browser sample. | OK |
| 14 | 6 | Body | It uses an isolated page-memory sample. | F-2-24 |
| 15 | 13 | Body | Use Reset demo to restore it, or Start for real to leave it. | OK |
| 16 | 3 | H2 | Use in CI | OK |
| 17 | 9 | Body | Create an export with one key name per line. | OK as an instruction |
| 18 | 7 | Body | Blank lines and `#` comments are allowed. | F-2-9: unlisted claim |
| 19 | 11 | Body | `delete_policy` controls extra keys: `block` fails, `warn` reports, and `allow` reports. | F-2-10: unlisted claim |
| 20 | 7 | Body | `--strict-extra` makes extras fail in every policy. | F-2-11: unlisted claim |
| 21 | 5 | Body | Exit code `0` means pass. | OK; listed claim |
| 22 | 5 | Body | Exit code `1` means drift. | OK; listed claim |
| 23 | 6 | Body | Exit code `2` means invalid input. | OK; listed claim |
| 24 | 3 | H2 | Privacy and scope | OK |
| 25 | 6 | Body | The CLI reads local key-name files. | F-2-12: unlisted claim |
| 26 | 7 | Body | It has no telemetry or provider integration. | F-2-13 and F-2-25 |
| 27 | 9 | Body | It does not create, update, or delete provider secrets. | F-2-14: unlisted claim |
| 28 | 10 | Body | Keep local reports restricted because key names can be sensitive. | OK as safety advice |
| 29 | 8 | Body | Input must contain one key name per line. | F-2-15: unlisted claim |
| 30 | 12 | Body | A line such as `KEY=value`, JSON, or a whitespace-delimited record is rejected. | F-2-16 and F-2-26 |
| 31 | 13 | Body | Valid key characters are ASCII letters, digits, `_`, `-`, `.`, `/`, and `:`. | F-2-17: unlisted claim |
| 32 | 3 | H2 | Develop and verify | OK |
| 33 | 6 | Body | The static site builds to `dist/site/`. | F-2-18: unlisted claim |
| 34 | 14 | Body | Deploy it as a static site; `staticwebapp.config.json` supplies headers and the designed 404 response. | F-2-19: unlisted claim |
| 35 | 1 | H2 | License | OK |
| 36 | 1 | Body | MIT. | Confirmed by `LICENSE` |
| 37 | 2 | Body | See LICENSE. | OK |

### Reachable dynamic copy checked separately

The live invalid-input and clean-result states reproduced F-2-27 through F-2-31 exactly. The clipboard fallback in F-2-32 is present in the deployed source. No button uses “Submit”, “Go”, or “Continue”; the action labels are result-naming verbs.

## Demo and sandbox evidence

- One click from the landing action reached `/demo/` and immediately showed `API_URL`, `DATABASE_URL`, `SESSION_KEY`, `SESION_KEY`, `OLD_WEBHOOK_TOKEN`, “Unsafe to deploy”, one missing, two extra, one rename, and 4 keys / 3 maximum.
- The persistent banner contained “Demo — sample data, nothing is saved”, Reset demo, and Start for real.
- Editing and resetting restored the exact sample. Preloaded `real:sentinel` values in localStorage and sessionStorage survived entry, Reset, and Start for real unchanged. No cookie or new storage key appeared.
- Every observed live request stayed on `https://secret-sync-preflight.sociobot.in`. After service-worker readiness and one online reload, an offline reload returned 200 and the report remained usable.
- In a fresh working directory, the clean-clone binary ran `sspf demo`, returned the documented drift exit 1, printed a new `/tmp/sspf-demo-*` directory, and produced three files byte-for-byte equal to `examples/`. The invocation created nothing in the working directory.
- The missing real-CLI landing recording remains F-2-1.

## Claim test results from a clean clone

Clean clone: `/tmp/tmp.JxJi1lj1I5/clone` at `3bc558f0ef5b24450a1938f9c16ee837997ee623`. All listed commands passed:

| Claim ID | Exact command | Result |
| --- | --- | --- |
| `demo-command` | `cargo test --test claims claim_demo_command_uses_bundled_key_only_files` | PASS, 1 test |
| `drift-report` | `cargo test --test claims claim_check_finds_the_advertised_drift_states` | PASS, 1 test |
| `no-secret-values` | `cargo test --test claims claim_values_are_rejected_without_echoing_them` | PASS, 1 test |
| `ci-exit-codes` | `cargo test --test claims claim_exit_codes_describe_pass_drift_and_input_error` | PASS, 1 test |
| `report-formats` | `cargo test --test claims claim_json_github_and_local_report_are_observable` | PASS, 1 test |
| `demo-browser-isolation` | `npm run test:claims -- --grep @claim:demo-browser-isolation` | PASS, desktop and mobile |
| `demo-browser-network` | `npm run test:claims -- --grep @claim:demo-browser-network` | PASS, desktop and mobile |

Passing commands do not close F-2-2 or the unlisted-claim findings because their assertions and inventory are incomplete.

## Earlier-finding verification

| Earlier finding | Live and code result |
| --- | --- |
| review-1 BLOCKING 1 | **Fixed.** The first screen names the job, audience, first action, and click result on both viewports. |
| review-1 BLOCKING 2 | **Half-fixed; reopened as F-2-1.** Browser/CLI demo mechanics work, but the required real-CLI recording is absent. |
| review-1 BLOCKING 3 | **Half-fixed; reopened as F-2-2 through F-2-19.** A ledger exists and commands pass, but public claims remain unlisted and several tests are incomplete. |
| review-1 BLOCKING 4 | **Fixed.** `/demo` and `/demo/` work; unknown paths return the designed page with HTTP 404. |
| review-1 MAJOR 1 | **Half-fixed; blocking again as F-2-20.** Core metadata/shared shell improved, but route focus, full Twitter metadata, and the 404 shell remain incomplete. |
| review-1 MAJOR 2 | **Half-fixed; blocking again as F-2-21.** Landing terms are consistent; CLI help/output still use desired/current. |
| review-1 MAJOR 3 | **Half-fixed; blocking again as F-2-22 through F-2-32.** Length and most copy improved, but the listed clarity defects remain. |
| review-1 MINOR 1 | **Confirmed.** Rendered links resolve and the parity-lattice identity remains distinct rather than a generic SaaS template. |

No `.factory/polish-*.md` files exist. The prior handoff and verification report were read and checked against the live site and source.

## Structure, links, and accessibility evidence

- `/`, `/demo/`, `/privacy/`, `/terms/`, the 404, assets, sitemap, robots file, and GitHub Source link returned expected 200/404 results. No dead link was found.
- Each tested route has `lang="en"`, one H1, one main landmark, ordered headings, a title matching the route, a meta description, a favicon, and no horizontal overflow at 390 px.
- The landing, Demo, Privacy, and Terms routes have canonicals and Open Graph data. The incomplete Twitter/404 metadata is F-2-20.
- Live Axe scans on five routes at mobile and desktop found zero serious or critical violations. Explicit size measurement found F-2-34, which Axe did not flag.
- The hero image has meaningful alt text. Fonts and scripts are self-hosted. The observed console and page-error logs were empty.
- The CSP and privacy-related response headers are present. Reduced-motion CSS removes smooth scrolling, transforms, and meaningful animation duration.
- The dark parity lattice, squared controls, mono status labels, phosphor/amber/coral signals, and non-looping resolve motion match `.factory/design.md` and are visually distinct.

## Quality gates

From the clean clone:

- `npm ci`: PASS, 0 vulnerabilities.
- Every exact command in `.factory/claims.json`: PASS.
- `npm test`: PASS — 3 Rust unit tests, 5 claim tests, 8 CLI integration tests, 1 doctest, and 14 Playwright cases.
- `npm run build`: PASS; created `target/release/sspf` and `dist/site/`.
- Built initial JS remains about 3 KB gzip across the landing modules and far below the budget.

## Missed leverage

No additional AI feature is justified. The job is a deterministic comparison of key names, and adding model inference would weaken privacy and predictability. The CLI already imports destination text exports, emits terminal/JSON/GitHub output, and supports CI blocking. No separate AI, import/export, or sync finding is raised beyond the missing real-CLI demonstration.

## What would make this perfect

Add the real `sspf demo` recording; bring every public promise into the ledger with complete observable tests; finish route focus, announcements, metadata, and the shared 404 shell; use expected/destination terminology in CLI output; apply every copy rewrite; replace the first-screen facts with tested privacy/offline/price facts; enlarge all touch targets; and complete the landing privacy/scope skeleton. Then rerun this entire review from a clean clone and fresh live contexts.
