# Secret Sync Preflight — independent verification

## PASS

Verified on **2026-08-28 UTC** against candidate commit
`466f58041877e355b6fcf20e09dc34a463275b71` and
`https://secret-sync-preflight.sociobot.in`.

No product-code changes were made during this verification.

## Acceptance result

The candidate satisfies the researched smallest useful product: a local,
read-only, metadata-only CLI that compares declared key names with destination
exports; reports missing, extra/stale, likely-renamed, and provider-limit
states; blocks dangerous delete plans; emits machine/CI output; and never
prints rejected values.

### Clean checkout quality gates

- Initial checkout was clean and `HEAD` was exactly
  `466f58041877e355b6fcf20e09dc34a463275b71`.
- `npm ci` completed with **0 vulnerabilities**.
- `npm test` passed: Rust unit/integration tests (**3 unit, 8 CLI
  integration, 1 doctest**) plus TypeScript type-check/site build and
  Playwright (**9 passed, 1 intentional mobile-only counterpart skip**).
- `cargo fmt --check` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- Exact production build, `npm run build`, passed and created
  `target/release/sspf` and `dist/site/`.
- `cargo package` passed, producing
  `target/package/secret-sync-preflight-0.1.0.crate` (89.6 KB compressed).

### Independent CLI end-to-end checks

- `sspf --help` and `sspf --version` worked; version is `0.1.0`.
- Seeded multi-environment fixture and integration suite caught missing,
  stale/extra, likely-renamed, and over-limit states and returned exit 1.
- A clean three-key destination exactly at its configured limit returned JSON
  `status: pass`, exit 0, and `at_limit: true`.
- An extra key under `delete_policy = "warn"` returned exit 0; the same input
  with `--strict-extra --format github` returned exit 1 and emitted GitHub
  error annotations.
- A malformed export containing `ALPHA=not-to-be-logged` returned exit 2.
  Its stderr named only the path and line; it contained neither the assignment
  nor the value.
- The packaged `.crate` was unpacked into a fresh temporary consumer,
  installed with `cargo install --path ... --root ...`, and the installed
  `sspf 0.1.0` completed the JSON clean-check successfully.

### Live deployment, privacy, and browser checks

- Live deployment matches the candidate byte-for-byte for `index.html`,
  `sw.js`, hero, favicon, all three JS bundles, CSS, and legal pages. The
  local/live `index.html` SHA-256 is
  `7d78f8e87c39d7767b123fe7a3c5922dc492d8a9f44c05e743d968d2585c568d`.
- Live response headers: self-only CSP (`default-src`, `img-src`, `script-src`,
  `style-src`, `font-src`, and `connect-src` all `'self'`), HSTS,
  `Referrer-Policy: no-referrer`, `X-Content-Type-Options: nosniff`, and a
  restrictive permissions policy. Hashed `/assets/*` responses are
  `max-age=31536000, immutable`; documents/service worker are revalidated at
  30 seconds.
- Live desktop and 390×844 mobile Chromium checks found one `<h1>`, one
  `<main>`, `lang="en"`, zero horizontal overflow, zero console/page errors,
  zero serious/critical axe violations, same-origin-only automatic requests,
  and keyboard Tab reached the next demo field. Keyboard focus rendered a
  3px blueprint-blue outline. Reduced-motion media was honored (transitions
  reduced to 0.01 ms and hero transform removed).
- A real offline reload after service-worker control returned HTTP 200 from
  the cached shell and showed “Offline · demo still works.”
- Static scan and observed requests found no analytics, telemetry, cookies,
  browser storage, third-party scripts, CDNs, or outbound runtime requests.
  The only external URL is a user-activated GitHub source/install link.
- Lighthouse mobile against the live URL: **performance 94**, **accessibility
  100**, FCP **1.4 s**, LCP **1.6 s**, CLS **0.063**, TBT **260 ms**. The
  launch emitted a post-audit Chromium target-crash warning, but Lighthouse
  wrote a complete report with no run warnings; the score/metrics above are
  from that report. Browser/axe checks were repeated separately and passed.
- Production sizes: initial app JS 4.80 KB (2.20 KB gzip), CSS 12.58 KB
  (3.67 KB gzip), selected WOFF2 fonts 63.1 KB, hero WebP 46.6 KB — all within
  the stated 200 KB JS, 50 KB CSS, 120 KB font, and 300 KB hero budgets.

## Defects

| Severity | Result |
| --- | --- |
| Critical | None found |
| High | None found |
| Medium | None found |
| Low | None found |

## Notes

`verify-url.sh` is not present in this repository. Equivalent direct live
browser checks covered title, language, landmark, image-alt/axe, and
console/page-error behavior. No deployment-only failure reproduced; the live
site was healthy and matched the tested candidate.
