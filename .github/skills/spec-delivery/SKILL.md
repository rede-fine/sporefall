---
name: spec-delivery
description: "Use when implementing a feature from specs/001-mushroom-sorting-game/spec.md, plan.md, and tasks.md and you need to keep code, tests, docs, and visual QA aligned."
---

# Sporefall spec delivery

Use this workflow when a request references the repo spec, plan, or tasks.

## Steps

1. Read `specs/001-mushroom-sorting-game/spec.md`, `plan.md`, and `tasks.md` first.
2. Trace the current behavior in `crates/web_app/src/`, `crates/game_core/src/`, and `data_pipeline/src/` before editing.
3. Keep implementation aligned with the spec language, or update the spec/tasks in the same change when the feature design becomes concrete.
4. Preserve the split between:
   - `crates/game_core`: pure game rules
   - `crates/web_app`: browser runtime, rendering, inputs, API integration
   - `data_pipeline`: offline export and inspection utilities
5. When UI changes are involved, verify both the canvas flow and any surrounding HTML/CSS shell changes.
6. Keep tests and smoke checks in sync with player-visible behavior.

## Repo-specific reminders

- The canvas renderer already uses `ui.rs` viewport/layout helpers.
- Menu, pause, fact, level-complete, and game-over flows all have pointer-aware buttons.
- Built-in mushroom classification data lives in `crates/web_app/src/catalog.rs`; reuse it instead of inventing duplicate gameplay rules in the browser.
