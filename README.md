# Sporefall

Sporefall is a browser game prototype where mushrooms fall into one of four lanes and the player must move each mushroom into the correct bucket before it lands.

At the start of a run, the player chooses one bucket set (Ecological Role, Cap Color, Peak Season, or Culinary Type). During play, each mushroom has a target lane for the selected set. Correct classifications award points and are added to a visible basket. Incorrect classifications still stack in the chosen lane. A row clear occurs when all lanes are non-empty; one mushroom is removed from each lane and bonus points are awarded.

The current build also supports iNaturalist imports, adaptive fall-speed scaling based on recent accuracy, session-end species struggle stats, observation-photo gallery cycling for imported species, and tap-to-open species cards from the collection review screens.

## Implementation Overview

This project is a Rust workspace. The game is implemented in Rust and compiled to WebAssembly for the browser.

- `crates/game_core` (Rust library): pure game domain and rules.
- `crates/web_app` (Rust + wasm-bindgen/web-sys): browser runtime, keyboard/pointer input handling, animation loop, and canvas rendering.
- `index.html` + `styles.css` (static shell): page container, responsive shell layout, iNaturalist controls, leaderboard UI, and styling around the canvas.
- `Trunk.toml`: Trunk build/serve configuration for WASM web output.

## What Runs Where

### Rust domain logic (`crates/game_core`)

- `src/model.rs`
  - Core data types (`FallingMushroom`, `GameConfig`, `PlacementFeedback`, `GameError`).
- `src/rules.rs`
  - `GameState` state machine and gameplay rules:
  - spawning, lane movement, hard drop,
  - per-lane stacking,
  - scoring,
  - basket collection for correct placements,
  - row-clear behavior.
- `tests/falling_loop.rs`
  - Integration tests for lane bounds, stacking, basket rewards, and clear/score behavior.

This crate is framework-free and browser-free, so it can be tested with normal Rust tests.

### Rust WASM app layer (`crates/web_app`)

- `src/lib.rs`
  - WASM entry point (`#[wasm_bindgen(start)]`), obtains canvas context from the DOM,
  - registers keyboard and pointer input,
  - runs the `requestAnimationFrame` loop and calls update/render each frame.
- `src/app.rs`
  - App orchestration (`AppState`), menu vs gameplay phases,
  - fall progression timer,
  - spawning via catalog,
  - action routing (menu navigation, pointer hit targets, in-game movement/drop).
- `src/catalog.rs`
  - Mushroom dataset and category modes,
  - computes lane target based on selected category set.
- `src/input.rs`
  - Key mapping (`Arrow` keys, `Space`, `Enter`) into game actions.
- `src/render.rs`
  - All canvas drawing:
  - menu screen,
  - falling mushroom animation,
  - lane stacks,
  - category labels,
  - score/feedback,
  - basket count,
  - responsive desktop/mobile layouts,
  - fact / review / game-over overlays,
  - species-card overlay,
  - fallback mushroom sprites when a remote image is still loading.
- `src/settings.rs`
  - Runtime defaults (lane count, spawn lane, scoring values).

No JavaScript gameplay code is used. JS interop is limited to browser APIs exposed through `wasm-bindgen` and `web-sys`.

## Assets

- Bundled built-in species photos live in `assets/mushroom-images/`.
- The old duplicate `assets/images/mushrooms/` folder was removed.
- Imported iNaturalist photos are loaded dynamically at runtime and cached by image key.
- `assets/data/image_attributions.json` is still the manifest location for bundled-photo provenance work.

## Frame-to-Frame Flow

Each frame in the browser follows this sequence:

1. Browser calls `requestAnimationFrame` callback (in `web_app/src/lib.rs`).
2. `AppState::tick(dt)` advances fall progress and phase animations (in `web_app/src/app.rs`).
3. If a mushroom reaches the bottom, app triggers `GameState::hard_drop()` (in `game_core/src/rules.rs`).
4. Domain logic updates lanes, basket, and score.
5. Renderer redraws the full canvas from current state (in `web_app/src/render.rs`).

## Build and Run

Prerequisites:

- Rust toolchain with `wasm32-unknown-unknown` target
- Trunk

Commands:

```powershell
python leaderboard_service.py
cargo test -p game_core
cargo check -p web_app --target wasm32-unknown-unknown
trunk serve --port 8080
```

Then open:

- `http://127.0.0.1:8080/`

The leaderboard service persists scores in `runtime-data\leaderboard.sqlite3` and serves the local API at
`http://127.0.0.1:8787/api/leaderboard`.

The game can be played with keyboard controls or by clicking/tapping the in-canvas buttons and lane targets.
