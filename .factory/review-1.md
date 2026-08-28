# Adversarial first-read review 1 — Secret Sync Preflight

## Verdict: FAIL

Reviewed 2026-08-28 UTC at `https://secret-sync-preflight.sociobot.in` in fresh Chromium contexts at 390×844 and 1440×900, and against repository commit `33a35a2736729996505d391cf2ea3e41cde9620f`.

There are four BLOCKING findings. A PASS requires zero blockers and at most three minor findings.

## Findings, ordered by severity

### BLOCKING 1 — the first screen does not say who this is for or give one clear first action

**Quote:** “Know your secrets line up before deploy.” / “Compare declared key names with every destination.” / primary action “Install the CLI” / secondary action “Run a local demo”.

**Cold read, before scrolling:**

- What it does: I infer that a CLI compares expected secret-key names with copies from other systems and warns before deployment.
- For whom: I cannot answer. The first screen never names DevOps teams, CI maintainers, or the situation of moving configuration across services. “Stable CI exits” is a technical feature, not an audience statement.
- What to click first: I cannot answer. The visual primary action asks me to install, while the try-out is secondary and does not say it contains sample data. On the 390 px header, “Install” is the only visible navigation action.

This fails the mandatory first-screen test on both viewports. The 22-word description uses “declared”, “destination”, “stale”, and “provider” before establishing the user or workflow.

**Concrete fix:** use one first path and name its result:

- H1: “Check secret key drift before deployment”
- Supporting sentence: “For DevOps teams syncing configuration across CI and hosting services.”
- Primary action: “Try it with sample data”
- Adjacent explanation: “Opens a seeded drift report in this browser.”
- Facts: “Reads key names only” / “Makes no provider calls” / “Free and open source”

Add claim entries and tests before publishing the three facts.

### BLOCKING 2 — the demo is not a CLI demo and has no sandbox controls or direct demo route

**Quote:** “Run a local demo” and “Local browser demo”.

The one-click anchor does reveal realistic sample keys and an already-computed “Unsafe to deploy” result. That is useful, but it is a separate browser reimplementation of a CLI. It does not meet the CLI demo contract:

- `https://secret-sync-preflight.sociobot.in/demo` returns the landing page at scroll position 0; it does not enter the demo.
- The live demo has no “Demo — sample data, nothing is saved” banner, “Reset demo”, or “Start for real”.
- There is no self-hosted terminal recording of the real binary.
- In a fresh temporary directory, `target/debug/sspf demo` exits 2 with `error: unrecognized subcommand 'demo'`.
- `.factory/demo.md` is absent.
- The README has no stable demo URL or demo command.

The browser implementation did leave `real:sentinel` values in localStorage and sessionStorage unchanged, and it created no storage keys. That confirms the current page is memory-only; it does not replace the missing explicit demo mode and CLI path.

**Concrete fix:** ship bundled examples plus `sspf demo`, which copies them into a temporary directory, runs the real comparison, and prints that directory. Add a self-hosted recording of that command. Make `/demo` open a seeded result immediately, keep the required banner visible, implement Reset and Start for real, and document the command, URL, reset behavior, and namespace in `.factory/demo.md` and README.

### BLOCKING 3 — the claims contract is absent; every public claim is unlisted

**Quote:** `.factory/claims.json` does not exist.

There were no listed claim commands to run. `npm test` passed (3 Rust unit tests, 8 CLI integration tests, 1 doctest, 9 Playwright tests, and 1 intentional skip), but no test is tagged `@claim:<id>` and the current suite is not a substitute for the required inventory. Visitors cannot trace any promise to one clean-sandbox test.

Every row below is an unlisted-claim finding. Add one claims entry and one observably tagged test per claim, or remove/soften the sentence.

| ID | Exact public claim | Test the entry must require |
| --- | --- | --- |
| C01 | “Metadata only · Read only” | Run the CLI on read-only fixtures; compare all fixture hashes before/after and inspect system/network activity. |
| C02 | “Compare declared key names with every destination.” | Seed multiple destinations and assert each appears in the report. |
| C03 | “Catch gaps, stale keys, likely renames, and hard provider limits—without touching a secret value.” | Assert all four states and prove a sentinel value is neither retained nor emitted. |
| C04 | “No tokens” | Run the documented demo with no credential environment variables. |
| C05 | “No network calls” | Deny/intercept network for the full CLI and browser demo flows. |
| C06 | “Stable CI exits” | Assert documented exit codes for pass, drift, and invalid input. |
| C07 | “Missing keys” / “Excess keys” / “Likely renames” / “Provider limits” / “Delete policy” | Assert the observable report for each advertised check. |
| C08 | “The tool compares metadata and stops there.” | Trace file reads/writes and assert only permitted key-name inputs are read. |
| C09 | “Get human, JSON, or GitHub output and a non-zero exit before unsafe deployment.” | Assert all formats and the blocking exit. |
| C10 | “Only key names are accepted.” / “Values and assignments are rejected.” | Feed assignments and structured records; assert rejection without value echo. |
| C11 | “Nothing leaves this tab or survives a refresh.” / “input stays in memory.” | Intercept requests, inspect storage, reload, and assert input disappears. |
| C12 | “No changes were made.” | Hash all demo inputs and storage before/after a run. |
| C13 | “One binary. Zero runtime services.” | Inspect the packaged artifact and run it with outbound network denied. |
| C14 | “Release artifacts are published by the Param Factory.” | Link to and request an actual release artifact, or change this to an accurate availability statement. |
| C15 | “Keep reports local.” | Assert report output only reaches the explicit local path. |
| C16 | README: “It compares … catches … and blocks dangerous delete plans before deployment.” | Assert missing, extra, rename, limit, and delete-policy blocking from bundled fixtures. |
| C17 | README: “It never stores, fetches, or prints secret values.” | Use sentinel values and trace output, files, memory boundary, and network. |
| C18 | README: “Build the single binary from source with Rust 1.85 or newer.” | Build with the minimum supported toolchain and assert one executable artifact. |
| C19 | README: “Blank lines and lines starting with `#` are ignored.” | Include both in a fixture and assert the same report as the clean fixture. |
| C20 | README: all three `delete_policy` behavior bullets | Assert `block`, `warn`, and `allow`, with and without `--strict-extra`. |
| C21 | README: the exit-code sentence | Assert exact codes 0, 1, and 2 from clean fixtures. |
| C22 | README: “Likely renames are suggestions only; `sspf` never mutates a provider.” | Assert advisory output and no input/provider mutation. |
| C23 | README: “`sspf` rejects dotenv assignments, JSON, and whitespace-delimited records …” | Test all three input classes and prove the values never appear. |
| C24 | README: every manifest-reference constraint (`version`, uniqueness, empty arrays, relative exports, limits, policies, ignore) | Parameterize tests for each documented rule. |
| C25 | README: “Keys may contain ASCII letters, digits, `_`, `-`, `.`, `/`, and `:`.” | Exercise every allowed character and adjacent rejected characters. |
| C26 | README: “Output is local unless the caller explicitly sends it elsewhere.” | Deny network and trace writes for terminal and report modes. |
| C27 | README: “It does not use analytics, network requests, cookies, or storage; demo input stays in the tab and is discarded on refresh.” | Intercept the whole browser flow, inspect cookies/storage, and reload. |
| C28 | README: “It does not connect to providers, accept provider tokens, rotate values, replace a secret manager, or automatically delete anything.” | Run with decoy credentials and monitored fixtures; assert no network or mutation. |
| C29 | README: “If an export contains an assignment … the error identifies only the line number.” | Feed a unique sentinel assignment and assert only the line number is emitted. |

Independent spot checks found same-origin-only browser requests, preserved storage sentinels, discarded edited input on reload, and a successful offline reload. These are useful evidence for future C05/C11/C27 tests, not claim registration.

### BLOCKING 4 — `/demo` and unknown deep links are broken routes; there is no designed 404

**Quote:** requesting `/definitely-not-a-real-route` returns HTTP 200 with the landing title and H1. Requesting `/demo` does the same.

This makes invalid URLs look valid and prevents the documented demo-style deep link from entering the product. The sitemap lists only `/`, `/privacy/`, and `/terms/`. There is no 404 route or styled recovery action. Broken routing is blocking under the review rubric.

**Concrete fix:** add a real `/demo` route with title “Demo — Secret Sync Preflight”, seeded state, route announcement, and H1 focus. Serve a designed 404 with a non-200 status and “Return home”. Add valid routes to the sitemap and test direct load, reload, back/forward, focus transfer, and unknown paths.

### MAJOR 1 — metadata and the cross-route skeleton are incomplete

The landing title, description, `lang`, one H1, `<main>`, favicon, and security headers are present. However, all routes omit canonical, Open Graph, Twitter-card, and apple-touch declarations. `/privacy/` and `/terms/` use a reduced header/footer rather than the shared header; the main header has no Privacy link. No footer includes “Built by Param Factory” or a version/build ID. Full-document navigation leaves focus on the body rather than explicitly moving it to the new H1.

**Concrete fix:** add route-specific canonical and social metadata plus a product-derived 1200×630 image and 180 px apple-touch icon. Use the same header/footer skeleton on every route. Add Privacy to the header, factory attribution and build ID to the footer, and a tested focus/announcement routine for route changes.

### MAJOR 2 — inconsistent terms make the comparison model harder to learn

The same concepts change names:

| Concept | Terms used | Use consistently |
| --- | --- | --- |
| Keys that should exist | declared, desired, expected, source of truth, contract | expected keys |
| Keys found at a service | current, destination export, provider export, key-only export | destination keys / destination export |
| Keys that should not exist | stale, excess, extra | extra keys |
| Running the check | comparison, preflight, guardrail | check |

**Concrete fix:** adopt the final column across the landing page, browser UI, CLI help, and README.

### MAJOR 3 — copy has three overlong sentences and several contextless or jargon-heavy lines

The full audit is below. There are no banned marketing words. Three README sentences exceed 22 words. The principal unclear headings are “The truth is a contract, not a console.” and “One binary. Zero runtime services.” The result label “Result / production” is misleading inside seeded browser data. “Run preflight” names an internal process rather than the result.

**Concrete fix:** apply the rewrites marked in the audit. In particular, use “Compare expected and destination keys” for the comparison heading, “Show drift report” for the button, “Sample result” for the result label, and “Runs locally as one CLI” for the install heading.

### MINOR 1 — navigation links are alive, and the identity is distinct

This is a verification note, not praise: every rendered link on `/`, `/privacy/`, and `/terms/` returned 200 or resolved to an existing in-page anchor, including the external source link. The dark parity-lattice art, squared controls, mono labels, and signal palette are recognizably product-specific rather than a generic gradient-card SaaS template. No finding is raised for dead links or generic visual identity.

## Copy audit

Word counting treats hyphenated compounds as one word and excludes punctuation-only symbols. UI fragments, metadata, headings, and reachable dynamic states are included because they must make sense when encountered alone. Code samples and sample key values are excluded. “OK” means no length, jargon, terminology, heading-context, marketing-word, or button-label issue was found in that unit. Claim registration is audited separately above.

### Live landing page

| # | Words | Exact copy | Copy finding / proposed rewrite |
| ---: | ---: | --- | --- |
| 1 | 4 | Metadata only · Read only | Jargon/ambiguous. Use “Reads key names only”. |
| 2 | 7 | Know your secrets line up before deploy. | Does not name the concrete job. Use “Check secret key drift before deployment”. |
| 3 | 7 | Compare declared key names with every destination. | “Declared” and “destination” arrive without context. Use “Compare expected keys with keys in each CI or hosting service.” |
| 4 | 15 | Catch gaps, stale keys, likely renames, and hard provider limits—without touching a secret value. | Inconsistent “stale”; jargon “provider limits”. Use “Find missing, extra, renamed, and over-limit keys without reading secret values.” |
| 5 | 3 | Install the CLI | Result-naming verb, but wrongly primary for a cold try-out. Keep as the secondary action. |
| 6 | 4 | Run a local demo | Does not promise sample data. Use “Try it with sample data”. |
| 7 | 2 | No tokens | Ambiguous. Use “No provider login”. |
| 8 | 3 | No network calls | OK in plain words; register the claim. |
| 9 | 3 | Stable CI exits | Jargon. Use “CI blocks unsafe deployments” and register it. |
| 10 | 3 | 01 Four layers. | Visual caption is unclear alone. Use “Four configuration layers.” |
| 11 | 2 | One contract. | Metaphor without context. Use “One expected key list.” |
| 12 | 2 | Missing keys | OK. |
| 13 | 2 | Excess keys | Inconsistent. Use “Extra keys”. |
| 14 | 2 | Likely renames | OK. |
| 15 | 2 | Provider limits | Use the chosen term: “Destination limits”. |
| 16 | 2 | Delete policy | OK for the target user after introduction. |
| 17 | 4 | A read-only safety layer | “Layer” is vague. Use “Checks key names without changing them”. |
| 18 | 8 | The truth is a contract, not a console. | Contextless metaphor. Use “Compare expected and destination keys”. |
| 19 | 15 | `sspf` keeps ownership explicit: your repository declares the expected names; provider exports describe what exists. | Dense and undefined product shorthand. Use “Your repository lists expected keys. Each destination export lists keys that exist.” |
| 20 | 7 | The tool compares metadata and stops there. | “Metadata” is vague. Use “The CLI compares key names and makes no changes.” |
| 21 | 1 | Declare | Use consistent term: “List expected keys”. |
| 22 | 11 | List expected key names per environment in a reviewable TOML manifest. | “Reviewable” is vague. Use “List each environment’s expected keys in a TOML file.” |
| 23 | 1 | Export | Too broad alone. Use “Export destination keys”. |
| 24 | 12 | Produce one key name per line with a provider’s read-only metadata command. | Jargon-heavy. Use “Export one key name per line with a read-only provider command.” |
| 25 | 2 | Block drift | “Drift” is introduced late. Use “Block unsafe deployment”. |
| 26 | 13 | Get human, JSON, or GitHub output and a non-zero exit before unsafe deployment. | “Human output” and “non-zero exit” are awkward. Use “Read a terminal, JSON, or GitHub report. Unsafe results exit with an error.” |
| 27 | 3 | Local browser demo | OK, but it is not the required CLI demo. |
| 28 | 4 | Run the comparison here. | Weak out of context. Use “Compare expected and destination keys”. |
| 29 | 5 | Only key names are accepted. | OK; register the claim. |
| 30 | 8 | Nothing leaves this tab or survives a refresh. | OK in plain words; register the claim. |
| 31 | 2 | Desired keys | Inconsistent. Use “Expected keys”. |
| 32 | 5 | One key name per line. | OK. |
| 33 | 5 | Values and assignments are rejected. | “Assignments” is jargon. Use “Entries such as `KEY=value` are rejected.” |
| 34 | 2 | Destination export | OK. |
| 35 | 8 | Paste a key-only export; input stays in memory. | “Key-only” and “in memory” are technical. Use “Paste one key name per line. Refreshing clears it.” |
| 36 | 2 | Provider limit | Inconsistent. Use “Destination limit”. |
| 37 | 2 | Delete policy | OK. |
| 38 | 2 | Run preflight → | Does not name the result. Use “Show drift report”. |
| 39 | 2 | Result / production | Mislabels sample data as production. Use “Sample result”. |
| 40 | 3 | Unsafe to deploy | OK. |
| 41 | 1 | Blocked | OK. |
| 42 | 1 | Missing | OK. |
| 43 | 1 | Extra | OK; make this the term everywhere. |
| 44 | 1 | Rename? | OK. |
| 45 | 1 | Capacity | “Limit” is the established concept. Use “Destination limit”. |
| 46 | 3 | 4 / 3 · over | Too terse. Use “4 keys / 3 maximum”. |
| 47 | 1 | Rename? | OK. |
| 48 | 4 | SESION_KEY → SESSION_KEY | OK as data. |
| 49 | 2 | Missing — SESSION_KEY | OK as data. |
| 50 | 4 | Extra — OLD_WEBHOOK_TOKEN · deletion block | “Deletion block” is compressed. Use “Extra: OLD_WEBHOOK_TOKEN. Deletion is blocked.” |
| 51 | 4 | Extra — SESION_KEY · deletion block | Same issue. Use “Extra: SESION_KEY. Deletion is blocked.” |
| 52 | 7 | Over limit — 1 key(s) beyond provider maximum | Bad plural token and inconsistent term. Use “Over limit: 1 key above the destination maximum.” |
| 53 | 4 | No changes were made. | OK; register the claim. |
| 54 | 11 | Review findings, then update the source of truth or destination export. | “Source of truth” is jargon. Use “Review the report. Then update the expected keys or destination export.” |
| 55 | 3 | Ship the guardrail | Metaphor. Use “Add the check to CI”. |
| 56 | 2 | One binary. | OK only with the following fragment; combine the heading. |
| 57 | 3 | Zero runtime services. | Jargon. Combined rewrite: “Runs locally as one CLI”. |
| 58 | 4 | Build from source today. | OK. |
| 59 | 8 | Release artifacts are published by the Param Factory. | “Artifacts” and the publisher are unexplained; the README says releases are separate. Use “Prebuilt downloads are not available yet” unless a real release link exists. |
| 60 | 3 | Copy install command | Result-naming verb; OK. |
| 61 | 6 | Secret names can be sensitive too. | OK. |
| 62 | 3 | Keep reports local. | “Local” is underspecified. Use “Store reports only on your CI runner.” |

#### Landing metadata, navigation, and dynamic states

| # | Words | Exact copy | Copy finding / proposed rewrite |
| ---: | ---: | --- | --- |
| M1 | 8 | Secret Sync Preflight — catch secret drift before deploy | Clear title pattern; use “deployment” instead of clipped “deploy”. |
| M2 | 16 | A read-only CLI that catches secret key drift, provider limits, and dangerous deletion plans before deployment. | “Provider limits” and “dangerous” need context. Use “A CLI that finds missing, extra, renamed, and over-limit secret keys before deployment.” |
| M3 | 3 | Skip to content | OK. |
| M4 | 3 | How it works | OK as navigation. |
| M5 | 2 | Live preflight | Jargon. Use “Sample check”. |
| M6 | 1 | Install | Result-naming verb; OK. |
| M7 | 1 | Privacy | OK. |
| M8 | 1 | Terms | OK. |
| M9 | 1 | Source | The external destination is not announced. Use “Source on GitHub (external)”. |
| D1 | 7 | Line [number] is not a key-only record. | “Key-only record” is jargon. Use “Line [number] must contain one key name.” |
| D2 | 4 | Remove values or spaces. | Explains the next action; OK. |
| D3 | 6 | Line [number] duplicates an earlier key. | Clear; OK. |
| D4 | 2 | Check input | Vague state. Use “Fix the key list”. |
| D5 | 8 | Provider limit must be from 1 to 10,000. | Use consistent term: “Destination limit must be between 1 and 10,000.” |
| D6 | 8 | Fix the highlighted key list, then run again. | Clear; OK. |
| D7 | 2 | Input error | Clear heading; OK. |
| D8 | 3 | Input needs attention | Vague. Use “Fix the key list”. |
| D9 | 10 | Assignments are rejected before values can appear in the result. | “Assignments” is jargon. Use “Entries such as `KEY=value` are rejected before the value is shown.” |
| D10 | 3 | Review before deploy | Use “Review before deployment”. |
| D11 | 3 | Safe to deploy | Clear; OK. |
| D12 | 1 | Warning | Clear; OK. |
| D13 | 1 | Passed | Clear; OK. |
| D14 | 1 | over | Too context-dependent. Use “over limit”. |
| D15 | 2 | at limit | Clear in the capacity row; OK. |
| D16 | 1 | within | Too context-dependent. Use “within limit”. |
| D17 | 1 | Aligned | Inconsistent with pass/safe terminology. Use “No drift”. |
| D18 | 8 | All declared keys are present; no excess keys found. | Inconsistent terms. Use “All expected keys are present. No extra keys were found.” |
| D19 | 2 | Ready offline | Ambiguous before a service worker controls the page. Use “Demo available offline”. |
| D20 | 4 | Offline · demo still works | Plain and clear; register the claim. |
| D21 | 1 | Copied | Clear button feedback; OK. |
| D22 | 5 | Install command copied to clipboard. | Clear; OK. |
| D23 | 2 | Copy unavailable | Clear state; OK. |
| D24 | 4 | Clipboard access was unavailable. | Repeats the heading. Use only the next action. |
| D25 | 7 | Select the command above to copy it. | Clear recovery action; OK. |

### README

| # | Words | Exact copy | Copy finding / proposed rewrite |
| ---: | ---: | --- | --- |
| 1 | 3 | Secret Sync Preflight | Heading is only the product name. Use “Secret key drift checks for deployment”. |
| 2 | 10 | `sspf` is a read-only parity check for secret key names. | “Parity” is jargon. Use “`sspf` compares expected secret key names with destination exports.” |
| 3 | 27 | It compares a desired manifest with key-only exports from CI and hosting destinations, catches missing, extra, likely-renamed, and over-limit states, and blocks dangerous delete plans before deployment. | **Over 22 words** and terminology-heavy. Use “It compares expected keys with CI and hosting exports. It reports missing, extra, renamed, and over-limit keys. Unsafe results block deployment.” |
| 4 | 8 | It never stores, fetches, or prints secret values. | OK; register the claim. |
| 5 | 18 | Built for DevOps teams whose configuration crosses IaC, a secret manager, CI, and one or more hosting providers. | “IaC” is unexplained. Use “Built for DevOps teams moving configuration through infrastructure code, secret managers, CI, and hosting services.” |
| 6 | 1 | Install | OK. |
| 7 | 12 | Build the single binary from source with Rust 1.85 or newer: | OK; register the version claim. |
| 8 | 6 | The factory publishes release binaries separately. | No link or availability state. Use “Prebuilt downloads are not available yet” or link the release. |
| 9 | 6 | This worker does not publish packages. | Internal factory voice. Remove it. |
| 10 | 1 | Usage | OK. |
| 11 | 3 | Create `preflight.toml`: | OK. |
| 12 | 10 | Create the provider export as one key name per line. | Use consistent term: “Create the destination export with one key name per line.” |
| 13 | 8 | Blank lines and lines starting with `#` are ignored: | OK; register the claim. |
| 14 | 3 | Run the preflight: | “Preflight” is product jargon. Use “Check the keys:”. |
| 15 | 3 | Useful output modes: | “Useful” adds nothing. Use “Output formats:”. |
| 16 | 9 | `delete_policy` controls how extra destination keys are classified: | OK. |
| 17 | 13 | `block`: an extra key is a dangerous deletion plan and fails the check. | “Dangerous” is asserted without explaining the action. Use “`block`: an extra key fails the check because removal needs review.” |
| 18 | 11 | `warn`: an extra key is reported but only fails with `--strict-extra`. | OK; register the claim. |
| 19 | 10 | `allow`: an extra key is informational unless `--strict-extra` is set. | “Informational” is abstract. Use “`allow`: an extra key is reported but does not fail unless `--strict-extra` is set.” |
| 20 | 30 | Exit codes are stable: `0` means safe to deploy, `1` means drift or an unsafe plan blocks deployment, and `2` means the manifest/export could not be read or validated. | **Over 22 words**. Use three bullets: “`0`: safe to deploy.” “`1`: drift blocks deployment.” “`2`: an input could not be read or checked.” |
| 21 | 10 | Likely renames are suggestions only; `sspf` never mutates a provider. | “Mutates” is jargon. Use “Rename matches are suggestions. `sspf` never changes provider data.” |
| 22 | 2 | GitHub Actions | OK. |
| 23 | 17 | The export files should be produced by a least-privilege, read-only metadata command and kept on the runner. | Dense security jargon. Use “Create exports with a read-only provider command. Keep them on the CI runner.” |
| 24 | 20 | Do not put values in an export: `sspf` rejects dotenv assignments, JSON, and whitespace-delimited records to prevent accidental value handling. | “dotenv” and “whitespace-delimited” are avoidable. Use “Do not add values. `sspf` rejects `KEY=value`, JSON, and lines containing spaces.” |
| 25 | 2 | Manifest reference | OK. |
| 26 | 4 | `version`: must be `1`. | OK. |
| 27 | 5 | `environments[].name`: unique display name. | Fragment. Use “`environments[].name`: a unique display name.” |
| 28 | 9 | `environments[].desired`: unique key names; empty arrays are valid. | Use consistent term: rename/document this as expected keys where compatibility permits. |
| 29 | 7 | `environments[].destinations[].name`: unique within its environment. | Fragment. Use “...: a name that is unique within its environment.” |
| 30 | 12 | `export`: path to a key-only text file, resolved relative to the manifest. | “Resolved” is jargon. Use “`export`: path to a key-name file, starting from the manifest’s folder.” |
| 31 | 6 | `limit`: optional positive provider key limit. | Use consistent term: “optional positive destination key limit.” |
| 32 | 10 | Desired or current count at/over the boundary is reported. | Compressed and inconsistent. Use “The report warns when expected or destination keys reach or exceed this limit.” |
| 33 | 7 | `delete_policy`: `block` (default), `warn`, or `allow`. | OK. |
| 34 | 11 | `ignore`: optional key names excluded from both desired and current comparisons. | Inconsistent. Use “...excluded from both expected and destination keys.” |
| 35 | 7 | Keys may contain ASCII letters, digits, `_`, `-`, `.`, `/`, and `:`. | “ASCII” is technical but necessary in a reference; OK. |
| 36 | 10 | Output is local unless the caller explicitly sends it elsewhere. | “Caller” is jargon. Use “Output stays on this computer unless you send it elsewhere.” |
| 37 | 17 | Because key names can still be sensitive, reports should be treated as build artifacts with restricted access. | “Build artifacts” is jargon. Use “Key names may be sensitive. Restrict access to saved reports.” |
| 38 | 1 | Development | OK. |
| 39 | 8 | The static documentation site includes an in-browser demo. | OK, but link it directly. |
| 40 | 21 | It does not use analytics, network requests, cookies, or storage; demo input stays in the tab and is discarded on refresh. | Two ideas and an unlisted claim. Use “The demo makes no external requests and uses no cookies or browser storage. Refreshing clears its input.” |
| 41 | 2 | Repository map | OK. |
| 42 | 6 | `src/` — Rust CLI and comparison engine. | “Engine” is vague. Use “Rust CLI and comparison logic.” |
| 43 | 7 | `tests/` — CLI integration and seeded fixture coverage. | “Fixture coverage” is testing jargon. Use “CLI end-to-end tests and sample inputs.” |
| 44 | 7 | `site/` — dependency-light Vite documentation and local-only demo. | “Dependency-light” is an unsupported adjective. Use “Vite documentation site and browser demo.” |
| 45 | 7 | `examples/` — safe key-only example manifest and exports. | “Safe” is an unlisted assertion. Use “sample manifest and key-name exports.” |
| 46 | 8 | `.factory/` — opportunity brief, visual system, and build handoff. | Internal detail but clear; OK. |
| 47 | 3 | Security and scope | OK. |
| 48 | 4 | This tool is metadata-only. | “Metadata-only” is vague. Use “This tool reads key names, not secret values.” |
| 49 | 19 | It does not connect to providers, accept provider tokens, rotate values, replace a secret manager, or automatically delete anything. | Long list but within cap. Split for faster reading: “It does not connect to providers or accept tokens. It never rotates or deletes secrets.” |
| 50 | 27 | If an export contains an assignment such as `KEY=value`, parsing stops before the value is retained or echoed, and the error identifies only the line number. | **Over 22 words**. Use “If an export contains `KEY=value`, parsing stops before retaining or printing the value. The error shows only the line number.” |
| 51 | 10 | Report security issues through the repository’s private security reporting channel. | The channel is not linked. Use a direct link or name the exact GitHub Security tab. |
| 52 | 13 | Do not include secret values or sensitive key names in a public issue. | OK. |
| 53 | 1 | License | OK. |
| 54 | 1 | MIT. | OK. |
| 55 | 2 | See LICENSE. | Link text is understandable; OK. |

## Other verification evidence

- Fresh live contexts: 390×844 and 1440×900; first-screen notes above were recorded before scrolling.
- Browser demo: sample desired keys `API_URL`, `DATABASE_URL`, `SESSION_KEY`; sample destination keys include `SESION_KEY` and `OLD_WEBHOOK_TOKEN`; initial result reports 1 missing, 2 extra, 1 likely rename, and 4/3 over limit.
- Privacy/offline probe: all observed requests used `https://secret-sync-preflight.sociobot.in`; local/session storage sentinels remained unchanged; offline reload returned HTTP 200 and “Offline · demo still works.”
- CLI demo probe: clean temp directory, exit 2, unrecognized `demo` subcommand, no product output files.
- Link crawl: all rendered links and anchors on landing, Privacy, and Terms resolved; no dead link found.
- Unknown route: HTTP 200 landing page, confirming the 404 defect.
- `npm ci`: passed, 0 vulnerabilities.
- `npm test`: passed; no claim-tagged tests exist.
