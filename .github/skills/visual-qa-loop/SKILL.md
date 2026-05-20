---
name: visual-qa-loop
description: "Use when visually checking the Sporefall page during feature work, especially for menu/layout changes, responsive canvas behavior, import UI, and gameplay feedback states."
---

# Sporefall visual QA loop

Use this workflow whenever a change affects the page layout, canvas rendering, or interaction flow.

## Steps

1. Build or serve the app before judging visuals.
2. Check the page in both desktop and compact layouts because `ui.rs` switches viewports based on available space.
3. Inspect these states explicitly:
   - menu
   - active gameplay
   - pause overlay
   - basket fact screen
   - level complete
   - game over
4. When a feature adds DOM UI outside the canvas, verify it does not break keyboard focus or pointer input inside the canvas.
5. If gameplay data changes image sources, confirm both loaded-image and fallback-sprite behavior still look acceptable.
6. Re-check spacing and legibility after every meaningful layout tweak instead of waiting until the end.

## Repo-specific reminders

- `styles.css` controls the outer shell; `render.rs` controls the actual game presentation.
- Pointer input must stay aligned with `ui.rs` hit rectangles after any viewport or canvas sizing change.
