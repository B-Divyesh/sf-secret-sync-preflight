# Demo sandbox

- Browser URL: `https://secret-sync-preflight.sociobot.in/demo/`. The landing action opens this route in one click.
- CLI command: `sspf demo`.
- Sample files: `examples/preflight.toml`, `examples/staging-ci.keys`, and `examples/production-hosting.keys`.
- Sample result: one missing key, two extra keys, one likely rename, and one destination over its limit.
- Browser isolation: sample edits stay in page memory. The demo writes no cookies, `localStorage`, `sessionStorage`, or IndexedDB data.
- Offline storage: the service worker caches product files only. It does not store sample edits.
- Reset: **Reset demo** restores the sample. Refreshing also clears edits.
- Exit: **Start for real** returns home without reading or changing existing browser storage.
- CLI isolation: `sspf demo` copies the bundled files to a new `sspf-demo-*` temporary directory and prints its path.
- Recording: `site/public/cli-demo.svg` and its text transcript reproduce the normalized output of the real demo command.
