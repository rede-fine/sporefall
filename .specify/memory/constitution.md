<!--
Sync Impact Report
Version change: template -> 1.0.0
Modified principles:
- Placeholder principle 1 -> I. Code Quality First
- Placeholder principle 2 -> II. Tests Prove Gameplay
- Placeholder principle 3 -> III. Consistent Player Experience
- Placeholder principle 4 -> IV. Performance Is a Feature
- Placeholder principle 5 -> V. Revertable Delivery
Added sections:
- Gameplay UX Standards
- Workflow & Review
Removed sections:
- None
Templates requiring updates:
- ✅ .specify/templates/plan-template.md
- ✅ .specify/templates/spec-template.md
- ✅ .specify/templates/tasks-template.md
- ⚠ No .specify/templates/commands directory exists
- ⚠ No repository README.md exists for runtime guidance sync
Follow-up TODOs:
- None
-->
# Sporefall Constitution

## Core Principles

### I. Code Quality First
Code MUST use small, named modules, explicit state transitions, and readable data
flow. Comments are allowed only when they capture short reasoning the code cannot
show on its own; they explain why, not what. Rationale: game rules will change
often, so the codebase must stay easy to reshape without hidden logic.

### II. Tests Prove Gameplay
Every player-visible rule, scoring change, control mapping, and configuration rule
MUST have automated coverage at the smallest practical level, plus a focused
journey check for the affected flow. Regressions MUST start with a failing test.
Rationale: interactive game behavior breaks silently unless rules are pinned down.

### III. Consistent Player Experience
Controls, feedback language, timing cues, and configuration behavior MUST stay
consistent across menus, levels, and bucket sets. A new mode MUST reuse the same
input model unless the plan records a stronger usability reason to diverge.
Rationale: players are here to learn mushroom classification, not a new interface
for each screen.

### IV. Performance Is a Feature
Interactive play MUST target smooth desktop browser performance, with 60 FPS as
the default goal and no avoidable per-frame work. Any feature touching rendering,
input, animation, or scoring feedback MUST name its hot path and its validation
check. Rationale: lag weakens both gameplay and learning.

### V. Revertable Delivery
Implementation MUST ship in narrow, user-visible slices that can be reverted
without losing unrelated work. Each implementation plan MUST suggest clean commit
boundaries with plain-language messages that explain what changed. Rationale:
gameplay iteration is safer when experiments can be backed out cleanly.

## Gameplay UX Standards

Every playable session MUST remain solvable from the clues shown to the player.
The active controls and bucket labels MUST be visible before play starts or a
level changes. Feedback for correct, incorrect, and score-changing actions MUST
be immediate and phrased consistently across the game.

## Workflow & Review

Plans MUST identify test scope, UX consistency risks, performance checks, and
intended commit slices before implementation starts. Tasks MUST include the tests
and validation work needed for each affected story, not just coding steps. Review
and implementation comments MUST justify non-obvious decisions briefly rather than
restating code.

## Governance

This constitution overrides informal local practice for Sporefall. Every plan,
task list, review, and implementation change MUST check compliance with these
principles. Amendments MUST update this file and any impacted templates in the
same change.

Versioning policy follows semantic versioning: MAJOR for removing or redefining a
principle, MINOR for adding a principle or materially expanding guidance, and
PATCH for clarifications that do not change project obligations.

Compliance review is mandatory at plan approval, before merge, and whenever a
performance-sensitive or player-facing rule changes.

**Version**: 1.0.0 | **Ratified**: 2026-05-19 | **Last Amended**: 2026-05-19
