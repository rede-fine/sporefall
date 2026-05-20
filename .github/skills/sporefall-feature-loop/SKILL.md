---
name: sporefall-feature-loop
description: "Implement and iterate on Sporefall features. Use when adding gameplay, leaderboard, shell, or workflow changes that need repo investigation, coordinated edits, local service setup, visual review, and final validation."
argument-hint: "[feature or goal]"
---

# Sporefall Feature Loop

Use this skill for end-to-end Sporefall changes that touch the Rust game, the WASM shell, the local leaderboard service, or repo workflows.

## Key Files

- `README.md`
- `leaderboard_service.py`
- `crates/web_app/src/app.rs`
- `crates/web_app/src/render.rs`
- `crates/web_app/src/leaderboard.rs`
- `index.html`
- `styles.css`

## Workflow

1. Read `specs/001-mushroom-sorting-game/plan.md`, `README.md`, and the files closest to the requested feature.
2. Trace the gameplay flow before editing:
   - rules in `crates/game_core`
   - app state in `crates/web_app/src/app.rs`
   - canvas output in `crates/web_app/src/render.rs`
   - shell markup in `index.html` and `styles.css`
3. If the change touches persistence or score flows, include `leaderboard_service.py` and `crates/web_app/src/leaderboard.rs` in the investigation set.
4. Keep edits coherent across the service, the DOM shell, and the WASM app so the UI and data flow stay aligned.
5. For local iteration, run:
   - `python leaderboard_service.py`
   - `trunk serve --port 8080`
6. Visually inspect the page after each meaningful UI change and refine spacing, copy, and responsive behavior before concluding.
7. Finish with the existing repo validation commands and note any environment limitation if local execution is unavailable.

## Guardrails

- Prefer small, persistent fixes over temporary fallbacks.
- Keep leaderboard behavior global unless the user explicitly asks for per-mode rankings.
- Preserve the canvas gameplay loop; add shell UI around it instead of moving core play into the DOM.
