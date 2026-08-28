# Demo sandbox

- Browser URL: `https://secret-sync-preflight.sociobot.in/demo/`; `/?demo=1` redirects there for the first-screen one-click path.
- CLI command: `sspf demo`.
- Sample: `examples/preflight.toml`, `examples/staging-ci.keys`, and `examples/production-hosting.keys`. It shows a missing key, extra keys, a likely rename, and a destination limit.
- Browser isolation: the sample begins from page constants and uses no cookies, `localStorage`, `sessionStorage`, IndexedDB, or network requests. **Reset demo** restores the constants. **Start for real** returns home.
- CLI isolation: `sspf demo` copies the bundled sample to a fresh `sspf-demo-*` temporary directory, prints its location, then runs the real comparison. It never reads a real manifest.
