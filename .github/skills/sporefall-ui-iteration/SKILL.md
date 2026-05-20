---
name: sporefall-ui-iteration
description: 'Implement or refine Sporefall canvas UI work. Use when changing render.rs, app.rs, lib.rs, input.rs, styles.css, mobile responsiveness, mouse support, touch support, menu flows, or gameplay interaction affordances.'
argument-hint: 'Describe the UI/input change to make'
---

# Sporefall UI Iteration

Use this skill for repeatable frontend work on the Sporefall game surface and surrounding shell.

## When to Use

- Responsive layout changes for desktop/mobile
- Mouse or touch interaction changes
- Canvas rendering updates
- Menu, pause, fact, level-complete, or game-over interaction work
- Coordination across `render.rs`, `app.rs`, `lib.rs`, `input.rs`, and `styles.css`

## Procedure

1. Read `specs/001-mushroom-sorting-game/spec.md`, `specs/001-mushroom-sorting-game/tasks.md`, and `specs/001-mushroom-sorting-game/plan.md`.
2. Inspect the active UI/input modules:
   - `crates/web_app/src/app.rs`
   - `crates/web_app/src/lib.rs`
   - `crates/web_app/src/render.rs`
   - `crates/web_app/src/input.rs`
   - `crates/web_app/src/ui.rs`
   - `styles.css`
   - `index.html`
3. Keep keyboard behavior intact while layering any pointer/mobile behavior on top.
4. Prefer shared layout helpers over duplicating hit-test coordinates in multiple files.
5. Update the spec/tasks/README when the shipped behavior changes or when a future item becomes implemented work.
6. Finish with a build/test pass and a visual check workflow.

## Project Notes

- The game canvas is rendered in Rust/WASM; interactive HTML outside the canvas currently includes iNaturalist import controls and leaderboard UI.
- Pointer parity means every game phase should remain operable without the keyboard.
- Mobile work should cover both the canvas layout and the page shell around it.
