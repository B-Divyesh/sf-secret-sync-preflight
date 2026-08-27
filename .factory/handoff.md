# Secret Sync Preflight — build handoff

Work order: `secret-sync-preflight-build-1`  
Version: `0.1.0`  
Completed: 2026-08-27

## What shipped

- `sspf`, a Rust single-binary CLI with a versioned, key-only TOML manifest.
- Comparison across multiple environments and destinations for missing, extra/stale, and likely-renamed keys.
- Destination capacity checks and explicit `block`, `warn`, or `allow` deletion policy.
- Stable CI exit codes: 0 safe, 1 blocked drift, 2 input/configuration failure.
- Human-readable terminal output, stable JSON output, GitHub Actions annotations, and optional local JSON report files.
- Strict export parser that accepts one key name per line and rejects assignments/structured records without echoing their contents.
- Protection against overwriting the manifest or an input export with `--report`.
- Seeded multi-environment fixtures and documented CI usage.
- Static landing/docs site at `dist/site/` with an equivalent local-only interactive preflight, responsive 390px layout, empty/error/offline states, keyboard path, and privacy/terms pages.
- Original 46 KB WebP “parity lattice” hero generated for this product. Full prompt, deployment provenance, palette, typography, spacing, and motion policy are recorded in `.factory/design.md`.
- Self-hosted Inter and IBM Plex Mono fonts, versioned service-worker shell cache, security/cache configuration, robots file, and sitemap. No analytics, remote scripts, runtime services, cookies, or browser storage.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo clippy --all-targets -- -D warnings
cargo package
```

- `npm test`: passed — 8 Rust/integration tests, 1 compiling doctest, and 9 passing Playwright assertions across desktop Chromium and 390×844 mobile Chromium (1 intentionally skipped desktop-only duplicate). The browser suite includes axe serious/critical checks, no-console-error checks, empty/error/offline behavior, keyboard order, seeded results, legal pages, and mobile overflow.
- `npm run build`: passed — release CLI at `target/release/sspf`; static deploy at `dist/site/` with `index.html` at its root.
- Clean-clone verification: passed with `npm ci && npm run build`; both expected artifacts were present.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo package`: passed; package created at `target/package/secret-sync-preflight-0.1.0.crate` (about 88 KB compressed). The factory may publish it; this worker did not.
- `npm audit`: 0 vulnerabilities.

## Lighthouse-class measurements

Measured against the production Vite preview in headless Chromium using Lighthouse 12.8.2 mobile defaults:

- Performance: **99**
- Accessibility: **100**
- Best practices: **100**
- SEO: **100**
- First Contentful Paint: **1.4 s**
- Largest Contentful Paint: **1.7 s**
- Total Blocking Time: **0 ms**
- Cumulative Layout Shift: **0.063**

Production asset sizes: app JavaScript 4.80 KB (2.20 KB gzip), CSS 12.58 KB (3.67 KB gzip), hero WebP 46 KB, selected WOFF2 fonts about 63 KB total. These are below the 200 KB JS, 50 KB CSS, 300 KB hero, and 120 KB font budgets.

## Known gaps and next steps

- Direct provider API integrations are intentionally out of scope for v1. Teams must create key-only exports with provider-specific read-only metadata commands.
- Likely rename detection uses a conservative edit-distance heuristic and is explicitly advisory; a human must confirm it.
- Release binaries, registry publication, and deployment are factory responsibilities. Recommended next step: publish signed binaries and add provider-specific export recipes to the docs as teams validate them.
