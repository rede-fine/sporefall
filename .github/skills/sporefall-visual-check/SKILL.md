---
name: sporefall-visual-check
description: 'Visually check Sporefall after UI changes. Use when verifying canvas layout, mobile readiness, menu readability, pointer affordances, responsive shell behavior, or before/after screenshot comparisons.'
argument-hint: 'Describe what visual state to verify'
---

# Sporefall Visual Check

Use this skill when you need a repeatable visual verification pass for the Sporefall page.

## When to Use

- After changing `render.rs`, `styles.css`, `index.html`, or UI/pointer behavior
- When checking desktop vs compact/mobile layout
- When confirming the page shell, canvas, and overlay states still read well
- When collecting screenshots for regression comparison

## Procedure

1. Start the local dependencies already used by the repo:
   - `python leaderboard_service.py`
   - `cargo test -p game_core`
   - `trunk serve --port 8080`
2. Open `http://127.0.0.1:8080/`.
3. Check at least:
   - menu readability
   - active play lane targets
   - pause/fact/level-complete/game-over buttons
   - iNaturalist form and leaderboard shell
   - narrow/mobile layout behavior
4. If Firefox is available, capture a headless screenshot:
   - `"C:\Program Files\Mozilla Firefox\firefox.exe" --headless --screenshot <output.png> http://127.0.0.1:8080/`
5. Record any layout regressions before making more UI edits.

## States Worth Verifying

- Menu with both columns visible
- Active gameplay with a falling mushroom
- Pause overlay
- Basket fact screen
- Level complete
- Game over
- Narrow/mobile viewport with the compact canvas preset
