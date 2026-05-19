# Implementation Plan: Sporefall Mushroom Sorting Game

**Branch**: `[001-mushroom-sorting-game]` | **Date**: 2026-05-19 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-mushroom-sorting-game/spec.md`

## Summary

Build Sporefall as a public GitHub Pages browser game using Rust compiled to
WebAssembly. Keep the site fully static, drive gameplay rules from a tested Rust
core, and feed the game with a curated regional mushroom dataset assembled from
scientific or institution-backed sources.

## Technical Context

**Language/Version**: Rust stable, targeting WebAssembly for the browser

**Primary Dependencies**: `wasm-bindgen`, `web-sys`, a small Rust game core,
and a static-site build flow suitable for GitHub Pages

**Storage**: Static JSON assets for curated mushroom data; browser-local storage
for future player preferences or progress

**Testing**: `cargo test` for gameplay rules and solvability checks; browser
smoke validation for the WASM shell

**Target Platform**: Modern desktop browsers hosted on GitHub Pages

**Project Type**: Static web application with Rust/WASM gameplay core and an
offline data-curation pipeline

**Performance Goals**: 60 FPS target during active play, score and feedback
updates visible within 1 second of row completion, compact first-load bundle

**Constraints**: No server runtime, public-web-compatible image licensing,
single-player only, all configured sessions must be solvable from selected clues

**Scale/Scope**: Curated regional v1 dataset covering a few dozen recognizable
macrofungi and the bucket systems defined in the feature spec

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Code quality: Keep gameplay rules in a Rust core crate and keep browser glue
  thin so rule changes stay testable and reversible.
- Testing: Start with failing-first checks for lane movement, hard drop,
  placement feedback, row clearing, score updates, and solvability gating.
- UX consistency: Arrow keys and space stay stable across all modes; bucket
  labels and correctness feedback remain visible and phrased consistently.
- Performance: Track input handling, falling-state updates, placement
  resolution, and render frequency as hot paths.
- Delivery: Ship in revertable slices with commit boundaries for scaffold,
  dataset schema, gameplay loop, configuration modes, and progression.

## Project Structure

### Documentation (this feature)

```text
specs/001-mushroom-sorting-game/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── checklists/
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── game_core/
│   └── src/
└── web_app/
    └── src/

assets/
├── data/
└── images/

data_pipeline/
└── [offline ingestion and curation tools]

tests/
├── integration/
└── regression/

index.html
styles.css
Cargo.toml
```

**Structure Decision**: Use a Rust workspace with one crate for pure gameplay
rules and one crate for browser bindings. Keep static assets and future data
pipeline work outside the WASM crate so the web bundle only ships game-ready
content.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Separate web and core crates | Keeps browser APIs out of gameplay rules | One crate would mix platform glue with logic and weaken testability |

## Suggested Commit Slices

- `chore: scaffold Rust workspace and browser shell`
- `docs: add Sporefall planning artifacts for Rust and data strategy`
- `data: define mushroom schema and provenance fields`
- `feat: implement falling-lane game core`
- `feat: connect WASM shell to core gameplay state`
