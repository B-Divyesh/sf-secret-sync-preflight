# Visual thesis — the parity lattice

## Direction

Secret Sync Preflight uses **generative geometry** as an explanatory system, not decoration. Configuration layers become offset key-shaped cells travelling through a strict lattice. Aligned cells are quiet and teal; gaps, excess cells, and a hard provider boundary interrupt the pattern. The visual tells the product story before it adds atmosphere: parity is ordered geometry, drift is a break in that order.

The site is deliberately single-mode, like an operations console in a dark equipment room. Dark surfaces also let the safety signals behave like illuminated status marks without resorting to a generic gradient hero.

## Palette

| Token | Value | Role |
| --- | --- | --- |
| `ink-950` | `#07110f` | page background |
| `ink-900` | `#0d1a17` | raised surfaces |
| `ink-800` | `#172622` | rules and secondary surfaces |
| `paper` | `#f2f5e9` | primary text |
| `mist` | `#a9b9af` | secondary text |
| `signal` | `#69f0b0` | primary action and clean state |
| `signal-dark` | `#063e2d` | text on signal |
| `amber` | `#ffcf70` | limits and warnings |
| `coral` | `#ff7b72` | missing/dangerous state |
| `blueprint` | `#83b9ff` | informational marks |

The hues come from the job: terminal phosphor, amber capacity lamps, red interlocks, and cyan blueprint ink. Measured foreground/background pairs meet WCAG AA; status always includes a word or symbol, never color alone.

## Type and spacing

- Display and prose: `Inter`, locally hosted as a compact WOFF2 subset; its open counters stay clear at mobile sizes.
- Code, labels, and metrics: `IBM Plex Mono`, locally hosted; tabular rhythm matches manifests and CI output.
- Type scale: 14, 16, 20, 28, 44, and a responsive 64px display step. Body never drops below 16px.
- Spacing follows an 8px base with 4px optical adjustments: 4, 8, 12, 16, 24, 32, 48, 64, 96.
- Corners are mostly 2–8px: tools and manifests are precise, not plush. Cards appear only for independently actionable demo/result regions.

## Interaction grammar

- The main action is a solid signal-green control; secondary actions are outlined.
- Focus is a 3px blueprint-blue ring with an offset, visible on every interactive element.
- In the live demo, each destination is a vertical lane. Results enter beside the field that caused them, and a capacity ruler shows the hard limit.
- Copy feedback replaces the button label for two seconds and is announced in a live region.
- Phone layout stacks the demo inputs and suppresses nonessential lattice fragments; no function disappears.

## Motion policy

The hero lattice resolves once on load: cells translate by no more than 12px and fade over 240–420ms, communicating layers moving into parity. Result rows use a 180ms opacity/translate transition. Nothing loops. Under `prefers-reduced-motion: reduce`, all transforms and smooth scrolling are removed and state changes are instantaneous.

## Original asset plan and provenance

- `site/public/parity-lattice.webp`: original generated editorial geometry for the hero, depicting stacked configuration planes, key-shaped cells, one gap, one extra cell, and a hard capacity boundary. It contains no text, logos, people, secret names, or recognizable vendor UI.
- Generation prompt: “Use case: stylized-concept. Asset type: wide landing page hero for a security-focused developer CLI. Primary request: abstract generative geometry showing four offset configuration planes as a precise isometric lattice; most key-shaped cells align, one clean empty slot and one displaced extra cell expose drift, with a thin amber capacity boundary. Style: editorial 3D relief, technical plotter precision, tactile dark graphite and matte ceramic, sparse phosphor green, amber and coral signals. Composition: wide, main structure centered-right with quiet negative space and a clean silhouette. Lighting: low-key studio, crisp edge light, deep ink background. Constraints: no text, no letters, no numbers, no logos, no people, no locks, no gradients, no watermark.”
- Generator: Factory Azure image deployment via `/opt/fleet/lib/gen-image.sh`, 1536×1024, high quality, 2026-08-27. Optimized locally to WebP with original PNG excluded from the repository.
- All icons and diagrams in the interface are hand-authored CSS/SVG primitives and project-owned under the repository’s MIT license.
- `site/public/cli-demo.svg`: hand-authored terminal recording generated from the real `sspf demo` output. A one-time progress rule uses SVG CSS; reduced-motion mode shows the completed state. The adjacent HTML transcript carries the same information without relying on text inside the image.
