---
name: sporefall-visual-review
description: "Review Sporefall visually. Use when checking page layout, leaderboard presentation, modal prompts, canvas scaling, responsive behavior, or when comparing screenshots after UI changes."
argument-hint: "[state or screen to inspect]"
---

# Sporefall Visual Review

Use this skill when the request depends on how Sporefall looks, not just whether the code compiles.

## Focus Areas

- Import panel readability and spacing
- Canvas sizing and card spacing
- Leaderboard density, ranking clarity, and empty/offline states
- Modal readability and input affordances
- Mobile stacking and sidebar behavior

## Workflow

1. Start the local services needed for the page:
   - `python leaderboard_service.py`
   - `trunk serve --port 8080`
2. Load the page and inspect the target state:
   - menu
   - active gameplay
   - game-over with qualifying score
   - leaderboard offline state
3. Capture or refresh screenshots whenever tooling is available, then review them with the image viewer.
4. Cross-check the rendered result against `index.html`, `styles.css`, and the relevant canvas rendering code.
5. Tighten spacing, text wrapping, alignment, and contrast until the new UI feels intentional rather than bolted on.

## Notes

- The leaderboard should remain readable even before any scores exist.
- The submission modal should block game controls while it is open.
- If screenshot tooling is unavailable, fall back to careful HTML/CSS inspection and compare against any existing screenshots in the repo.
