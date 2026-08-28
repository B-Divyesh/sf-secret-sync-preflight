# Secret Sync Preflight — polish round 1 handoff

## Result

Repair commit: `b34e87339197238749b51d511ea4768a4ac0869b` (based on review `11c332ade7a59c10219c3e5856ee7e622163beed`).

All four blocking findings in `.factory/review-1.md` were addressed. The parity-lattice visual system remains intact.

## Delivered

- Rewrote the first screen for DevOps teams, made **Try it with sample data** the first action, and retained a plain install path.
- Added `/?demo=1` redirect support and the real `/demo/` route with a seeded report, persistent sandbox banner, **Reset demo**, **Start for real**, no browser storage, and an offline precache.
- Added `sspf demo`. It writes the bundled key-name-only fixture to a fresh temporary directory, prints the path, and runs the real comparator.
- Added `.factory/demo.md`, `.factory/claims.json`, exact claim commands, and `tests/claims.rs`.
- Added canonical/social metadata, product-derived social card and touch icon, shared legal navigation/footer, sitemap demo entry, focus transfer after leaving demo, and a styled status-404 route.
- Removed the static fallback that made unknown URLs look valid. `staticwebapp.config.json` now rewrites 404s to `404.html` with status 404.
- Added a production-style local static server for route/status checks and browser tests for direct routes, query demo entry, reset/isolation, offline reload, keyboard, mobile overflow, axe serious/critical issues, and 404s.

## Verification evidence

Ran from this clean dependency install:

```sh
npm ci
npm test
npm run build
cargo package --allow-dirty
```

Results: `npm test` passed (3 Rust unit tests, 5 claim tests, 8 CLI integration tests, 1 doctest, and 14 Playwright checks across desktop/mobile). `npm run build` produced `target/release/sspf` and `dist/site/`. `cargo package --allow-dirty` packaged and verified the crate.

Every claims command in `.factory/claims.json` was run individually. Browser claim checks include same-origin request capture, no cookies/local/session storage, reset, and offline reload after first visit. Axe is run in the Playwright suite with zero serious or critical violations. The built demo JavaScript is 2,363 bytes gzip; CSS is 3,880 bytes gzip.

Manual CLI smoke check: `./target/release/sspf demo` returned its expected drift status `1`, printed a fresh `/tmp/sspf-demo-*` directory, and showed the missing, extra, rename, and limit sample results.

## Deploy

Static output is `dist/site/`; deployment is expected to use the repository static-site work-order integration. No deployment credentials or local deployment configuration were present in the repository.

## Known gaps

No known blocking findings remain. Prebuilt binary downloads are intentionally not offered; source install is documented.
