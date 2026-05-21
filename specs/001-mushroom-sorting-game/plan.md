# Implementation Plan: Sporefall Mushroom Sorting Game

**Branch**: `001-mushroom-sorting-game` | **Date**: 2026-05-20 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-mushroom-sorting-game/spec.md`

**Status**: Complete with post-ship gameplay and asset cleanup extensions

## Summary

A browser-based educational mushroom sorting game built in Rust compiled to WebAssembly. Players sort falling mushrooms into labeled bucket lanes using keyboard controls, progressing through 4 levels with different classification systems (Cap Color → Culinary Type → Ecological Role → Peak Season). Features a two-axis difficulty system (Game Mode × Variety), 28 curated mushroom species, center-screen feedback animations, a basket collection panel, educational facts on row clears, iNaturalist import support, adaptive difficulty scaling, review-time species statistics, and collection species cards.

## Technical Context

**Language/Version**: Rust (stable, wasm32-unknown-unknown target)

**Primary Dependencies**: web-sys, wasm-bindgen, game_core (internal crate)

**Storage**: Browser runtime state + local SQLite leaderboard service

**Testing**: cargo test (unit tests in game_core)

**Target Platform**: WASM via Trunk 0.21, deployed to GitHub Pages

**Project Type**: Browser game (static SPA)

**Performance Goals**: 60 FPS canvas rendering at 960×540

**Constraints**: Keyboard-only input, no server runtime, <100ms feedback latency

**Scale/Scope**: 28 bundled species plus variable imported iNaturalist species, 4 levels, single-player

## Constitution Check

*All principles satisfied in the delivered implementation.*

- **Code quality**: Small named modules (`model.rs`, `rules.rs`, `catalog.rs`, `render.rs`, `app.rs`, `input.rs`, `facts.rs`, `images.rs`). State transitions are explicit via `GamePhase` enum. Non-obvious design captured in type names (e.g., `PlacementFeedback.cleared_mushrooms`, `sorted_this_level` HashSet).
- **Testing**: 7 automated tests in `crates/game_core/tests/falling_loop.rs` cover spawn, movement, correct/incorrect placement, row clearing, retry queue, and scoring. Cheapest failing-first check: `cargo test -p game_core`.
- **UX consistency**: Same keyboard model across all screens (arrows + spacebar + ESC). Bucket labels visible before play. Feedback animations use consistent center-screen pattern (Correct/Wrong/BasketCollected).
- **Performance**: Hot path is `requestAnimationFrame` loop → `update()` → `render()`. Canvas draw calls are minimal (background + mushroom + lanes + UI text). No per-frame allocations in the render path. Validation: visual smoothness at 60 FPS desktop browsers.
- **Delivery**: Implemented in revertable slices (see Commit Slices below).

## Project Structure

### Documentation (this feature)

```text
specs/001-mushroom-sorting-game/
├── plan.md              # This file
├── research.md          # Technology & data source decisions
├── data-model.md        # Entity definitions and validation rules
├── quickstart.md        # Build & run instructions
├── spec.md              # Feature specification
└── tasks.md             # Implementation tasks
```

### Source Code (repository root)

```text
crates/
├── game_core/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs           # Public re-exports
│   │   ├── model.rs         # Core types: FallingMushroom, GameConfig, PlacementFeedback, BucketSet
│   │   └── rules.rs         # GameState machine: lanes, basket, retry queue, score, row clearing
│   └── tests/
│       └── falling_loop.rs  # 7 unit tests for game rules
├── web_app/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs           # WASM entry point with requestAnimationFrame loop
│       ├── app.rs           # AppState, GamePhase enum, update logic, level progression
│       ├── catalog.rs       # 28 mushroom species, Variety enum, CategoryMode, pick_mushroom
│       ├── render.rs        # Full canvas renderer (menu, playing, animations, level complete, game over)
│       ├── input.rs         # Keyboard + pointer event → InputAction mapping
│       ├── inat.rs          # iNaturalist API integration (gloo-net async)
│       ├── leaderboard.rs   # Async leaderboard client (gloo-net + spawn_local)
│       ├── ui.rs            # Responsive viewport/layout system (desktop + compact mobile)
│       ├── images.rs        # Image preloading for all 28 species
│       ├── facts.rs         # Educational facts for basket events
│       └── settings.rs      # PlayerSettings (lane_count, points config)
assets/
├── data/
│   ├── mushrooms.v1.json          # Curated mushroom dataset
│   ├── mushrooms.schema.json      # JSON Schema for dataset validation
│   └── image_attributions.json    # Image provenance and licensing
└── mushroom-images/               # Bundled built-in mushroom photographs
data_pipeline/
├── Cargo.toml
└── src/
    ├── main.rs                    # Pipeline entry point
    ├── records.rs                 # MushroomRecord types
    └── export.rs                  # Export to game format
tests/
└── integration/
    └── game_shell_smoke.rs        # Integration smoke test
index.html                         # Trunk entry point
styles.css                         # Base styles
Trunk.toml                         # Trunk build configuration
```

## Architecture

### Module Responsibilities

| Module | Responsibility |
|--------|---------------|
| `game_core::model` | Pure data types shared across the system |
| `game_core::rules` | State machine: spawn → move → hard_drop → row clear → retry |
| `web_app::app` | Application orchestration: phases, level transitions, animation timing |
| `web_app::catalog` | Static mushroom data, variety filtering, pick logic with no-repeat |
| `web_app::render` | Canvas 2D drawing for all game phases |
| `web_app::input` | Keyboard event → domain action translation |
| `web_app::images` | Async image preloading via web-sys |
| `web_app::facts` | Context-sensitive educational text generation |

### State Flow

```
Menu → Playing → (BasketFact | LevelComplete | GameOver)
                    ↓                ↓
                 Playing          Playing (next level)
```

### Key Design Decisions

1. **Separate game_core crate**: Rules are testable without WASM/browser dependencies.
2. **Static catalog**: 28 species hardcoded in Rust for zero-latency access and compile-time guarantees.
3. **PlacementFeedback carries cleared_mushrooms**: Enables fact screen and basket panel to show what was just collected without re-querying state.
4. **sorted_this_level HashSet**: Implements FR-012 (no-repeat mechanic) efficiently.
5. **CenterAnimation enum with progress field**: Unified animation model for all feedback types with time-based advancement.

## Commit Slices (Delivered)

1. **game_core: types and rules** — Core state machine with spawn, move, hard_drop, row clearing, retry queue
2. **game_core: unit tests** — 7 tests covering all rule paths
3. **web_app: catalog and settings** — 28 mushroom species, CategoryMode, Variety, PlayerSettings
4. **web_app: app state and phases** — GamePhase enum, AppState, update loop, level transitions
5. **web_app: canvas renderer** — Menu, playing field, animations, level complete, game over screens
6. **web_app: input, images, facts** — Keyboard handling, image preloading, educational facts
7. **web_app: WASM entry point** — lib.rs with requestAnimationFrame loop, Trunk config

## Complexity Tracking

No constitution violations. The architecture uses exactly 2 crates (game_core + web_app) plus a data pipeline tool, all justified:
- `game_core`: Testable rules without browser deps
- `web_app`: WASM target with web-sys rendering
- `data_pipeline`: Offline curation tool, not part of the game binary
