# Copy audit

Audited 5 September 2026. Word counts treat code tokens and hyphenated terms as one word. Command output is checked separately against the real CLI.

## First screen read aloud

“Check secret key drift before deployment. For DevOps teams syncing configuration across CI and hosting services. Try it with sample data.”

This states the job, audience, and first action in one breath. The action, its result, and all three practical facts appear before scrolling at 390×844.

| Words | Location | Exact copy | Result |
| ---: | --- | --- | --- |
| 7 | Title | Secret Sync Preflight — check secret key drift | Pass |
| 13 | Description | A local CLI for DevOps teams to find secret key drift before deployment. | Pass |
| 3 | Skip link | Skip to content | Pass |
| 3 | Navigation | How it works | Pass |
| 1 | Navigation | Demo | Pass |
| 1 | Navigation | Privacy | Pass |
| 1 | Navigation | Install | Pass |
| 3 | Label | Key names only | Pass |
| 6 | H1 | Check secret key drift before deployment. | Pass |
| 10 | Audience | For DevOps teams syncing configuration across CI and hosting services. | Pass |
| 5 | Primary action | Try it with sample data | Pass |
| 8 | Action result | Opens a sample drift report in this browser. | Pass |
| 3 | Secondary action | Install the CLI | Pass |
| 3 | Fact | No provider login | Pass |
| 7 | Fact | Demo works offline after the first visit | Pass |
| 5 | Fact | Free under the MIT License | Pass |
| 4 | Figure caption | Four configuration layers. | Pass |
| 4 | Figure caption | One expected key list. | Pass |
| 16 | Image alternative | Four dark configuration layers show one missing key, one extra key, and an amber destination limit. | Pass |

## Remaining landing page

| Words | Location | Exact copy | Result |
| ---: | --- | --- | --- |
| 2 | Check label | Missing keys | Pass |
| 2 | Check label | Extra keys | Pass |
| 2 | Check label | Likely renames | Pass |
| 2 | Check label | Destination limits | Pass |
| 2 | Check label | Delete policy | Pass |
| 3 | Recording label | Real CLI sample | Pass |
| 6 | Recording H2 | Watch the CLI find unsafe drift. | Pass |
| 4 | Recording body | Recorded from `sspf demo`. | Pass |
| 11 | Recording body | The command exits with code 1 because the sample contains drift. | Pass |
| 10 | Recording caption | The sample files are written to a new `/tmp/sspf-demo-*` folder. | Pass |
| 4 | Recording control | Read the CLI transcript | Pass |
| 2 | Section label | Key-name checks | Pass |
| 5 | How H2 | Compare expected and destination keys. | Pass |
| 5 | How body | Your repository lists expected keys. | Pass |
| 7 | How body | Each destination export lists keys that exist. | Pass |
| 9 | How body | The CLI compares those names without changing the files. | Pass |
| 3 | Step H3 | List expected keys | Pass |
| 10 | Step body | List each environment’s expected keys in a TOML file. | Pass |
| 3 | Step H3 | Export destination keys | Pass |
| 11 | Step body | Export one key name per line with a read-only provider command. | Pass |
| 3 | Step H3 | Block unsafe deployment | Pass |
| 8 | Step body | Read a terminal, JSON, or GitHub report. | Pass |
| 6 | Step body | Unsafe results use exit code 1. | Pass |
| 2 | Section label | Browser sample | Pass |
| 5 | Demo H2 | Edit a sample drift report. | Pass |
| 7 | Demo body | The sample is separate from your data. | Pass |
| 7 | Demo body | Reset restores its expected and destination keys. | Pass |
| 3 | Section label | Privacy and scope | Pass |
| 7 | Scope H2 | Know what the CLI does not do. | Pass |
| 9 | Scope body | It reads the local manifest and exports you provide. | Pass |
| 10 | Scope body | It never connects to a provider or changes provider data. | Pass |
| 3 | Scope item | No secret values | Pass |
| 9 | Scope body | Lines containing values are rejected without printing the value. | Pass |
| 3 | Scope item | No automatic changes | Pass |
| 12 | Scope body | The CLI reports drift but never updates or deletes a provider secret. | Pass |
| 2 | Scope item | Local reports | Pass |
| 10 | Scope body | A report file is created only when you use `--report`. | Pass |
| 5 | Install label | Add the check to CI | Pass |
| 5 | Install H2 | Install the CLI with Cargo. | Pass |
| 4 | Install body | Run the sample first. | Pass |
| 7 | Install body | Then point the check at your manifest. | Pass |
| 3 | Install action | Copy install command | Pass |
| 7 | Footer | Checks expected key names against destination exports. | Pass |
| 5 | Footer | Key names can be sensitive. | Pass |
| 5 | Footer | Built by Param Factory · v0.1.0 | Pass |
| 1 | Footer | Privacy | Pass |
| 1 | Footer | Terms | Pass |
| 3 | Footer | Source (opens GitHub) | Pass |

## Dynamic copy

| Maximum words | States checked | Result |
| ---: | --- | --- |
| 11 | invalid key line, duplicate key, invalid limit, and recovery instruction | Pass |
| 4 | Fix the key list | Pass |
| 3 | Unsafe to deploy / Safe to deploy | Pass |
| 3 | Review before deployment | Pass |
| 2 | No drift | Pass |
| 10 | missing, extra, rename, capacity, and policy finding messages | Pass |
| 4 | Preparing offline demo / Available offline | Pass |
| 5 | Offline · demo still works | Pass |
| 7 | clipboard success and recovery messages | Pass |

The README contains no sentence over 13 words. Route, legal, error, and button copy contains no sentence over 22 words.

No audited copy contains a banned marketing word. There are no metaphor headings or mood labels.

## Terminology

| Concept | Required term |
| --- | --- |
| Names that should exist | expected keys |
| Names found in a service export | destination keys |
| Names that should not exist | extra keys |
| Running the comparison | check |
| Saved machine-readable result | report |

The serialized version 1 manifest field remains `desired` for file compatibility. User-facing copy calls its contents “expected keys.”
