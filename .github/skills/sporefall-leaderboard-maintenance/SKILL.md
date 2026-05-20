---
name: sporefall-leaderboard-maintenance
description: "Maintain or debug the Sporefall leaderboard. Use when checking the SQLite file, resetting scores, verifying the local API, debugging submission flow, or updating leaderboard-related docs and workflows."
argument-hint: "[maintenance task]"
---

# Sporefall Leaderboard Maintenance

Use this skill for score persistence work, API debugging, and leaderboard reset or inspection tasks.

## Key Assets

- `leaderboard_service.py`
- `runtime-data\leaderboard.sqlite3`
- `crates/web_app/src/leaderboard.rs`
- `README.md`

## Workflow

1. Inspect the current leaderboard contract in `leaderboard_service.py` and the browser client in `crates/web_app/src/leaderboard.rs`.
2. Start or restart the local service with:
   - `python leaderboard_service.py`
3. Verify the API surface:
   - `GET /api/health`
   - `GET /api/leaderboard`
   - `POST /api/leaderboard`
4. When debugging ranking issues, confirm:
   - scores sort descending
   - ties remain stable by timestamp then id
   - only the Top 10 render in the UI
5. If the data needs a clean slate, stop the service and remove `runtime-data\leaderboard.sqlite3`, then restart the service to recreate the schema.
6. Keep the README and any workflow skills aligned with the current service commands and storage path.

## Guardrails

- Treat the database file as runtime state and keep it ignored by git.
- Do not add browser-only fallbacks that silently bypass the persistent leaderboard.
- Keep player-name validation consistent between the client prompt and the service.
