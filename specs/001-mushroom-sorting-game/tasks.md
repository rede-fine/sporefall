# Tasks: Sporefall Mushroom Sorting Game

**Input**: Design documents from `/specs/001-mushroom-sorting-game/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Tests**: Tests are required for gameplay rules, browser-shell behavior, dataset solvability, and regressions that affect player-visible behavior.

**Organization**: Tasks are grouped by user story to preserve independent delivery and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g. `[US1]`, `[US2]`, `[US3]`)
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Finish the shared Rust/WASM and Pages tooling required before feature work can ship.

- [x] T001 Configure Trunk application build settings in Trunk.toml
- [x] T002 Add GitHub Pages deployment workflow in .github/workflows/deploy-pages.yml
- [x] T003 [P] Add local cargo aliases and WASM target defaults in .cargo/config.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data and application foundations that block all user stories.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T004 Define the curated mushroom dataset schema in assets/data/mushrooms.schema.json
- [x] T005 [P] Scaffold the offline data pipeline crate in data_pipeline/Cargo.toml
- [x] T006 [P] Implement source-record normalization entrypoint in data_pipeline/src/main.rs
- [x] T007 [P] Create shared web application state container in crates/web_app/src/app.rs
- [x] T008 [P] Add browser settings persistence model in crates/web_app/src/settings.rs
- [x] T009 Create image attribution manifest structure in assets/data/image_attributions.json

**Checkpoint**: Rust workspace, build pipeline, and curated data foundations are ready for story work.

---

## Phase 3: User Story 1 - Sort Falling Mushrooms (Priority: P1) 🎯 MVP

**Goal**: Deliver the core falling-mushroom loop with keyboard control, placement feedback, row clearing, and score updates.

**Independent Test**: Start a local session, move a falling mushroom between lanes with arrow keys, press space to drop it, and verify feedback, row clearing, and score changes without relying on configuration or progression features.

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T010 [P] [US1] Add gameplay rule tests for lane movement, hard drop, and row clearing in crates/game_core/tests/falling_loop.rs
- [x] T011 [P] [US1] Add browser smoke test for canvas rendering and keyboard loop in tests/integration/game_shell_smoke.rs

### Implementation for User Story 1

- [x] T012 [P] [US1] Extract gameplay entities from the core crate into crates/game_core/src/model.rs
- [x] T013 [P] [US1] Implement deterministic falling-lane reducer logic in crates/game_core/src/rules.rs
- [x] T014 [US1] Implement keyboard input mapping for arrow keys and space in crates/web_app/src/input.rs
- [x] T015 [US1] Render lanes, falling mushrooms, and placement feedback in crates/web_app/src/render.rs
- [x] T016 [US1] Connect the browser shell to the core update loop in crates/web_app/src/lib.rs
- [x] T017 [US1] Style the in-game score and feedback overlays in styles.css

**Checkpoint**: User Story 1 is playable and testable as a standalone MVP slice.

---

## Phase 4: User Story 2 - Configure Learning Mode (Priority: P2)

**Goal**: Allow players to choose bucket sets and clue modes while guaranteeing the configured session remains solvable from curated data.

**Independent Test**: Open the setup flow, choose different bucket sets and clue combinations, start a game, and confirm only valid solvable combinations are allowed and reflected in live gameplay.

### Tests for User Story 2 ⚠️

- [ ] T018 [P] [US2] Add solvability and bucket-set validation tests in crates/game_core/tests/configuration_rules.rs
- [ ] T019 [P] [US2] Add dataset provenance regression checks in tests/regression/mushroom_dataset.rs

### Implementation for User Story 2

- [ ] T020 [P] [US2] Implement bucket-set and clue-mode configuration types in crates/game_core/src/configuration.rs
- [ ] T021 [P] [US2] Parse curated mushroom records and provenance fields in data_pipeline/src/records.rs
- [ ] T022 [US2] Generate game-ready card exports and validation reports in data_pipeline/src/export.rs
- [ ] T023 [US2] Build the clue and bucket selection screen in crates/web_app/src/menu.rs
- [ ] T024 [US2] Enforce solvable session configuration before game start in crates/web_app/src/app.rs
- [ ] T025 [US2] Add the first curated regional mushroom dataset in assets/data/mushrooms.v1.json
- [ ] T026 [US2] Add public image attribution entries for the initial dataset in assets/data/image_attributions.json

**Checkpoint**: User Story 2 is independently functional with curated data and solvability protection.

---

## Phase 5: User Story 3 - Learn Through Progression (Priority: P3)

**Goal**: Add level-based bucket rotation and progression while keeping controls, scoring, and feedback consistent.

**Independent Test**: Complete early rounds, advance levels, and confirm bucket sets change with explicit level messaging while the same controls and scoring rules continue to apply.

### Tests for User Story 3 ⚠️

- [ ] T027 [P] [US3] Add level progression and bucket rotation tests in crates/game_core/tests/level_progression.rs
- [ ] T028 [P] [US3] Add browser journey smoke test for level transition messaging in tests/integration/level_transition_smoke.rs

### Implementation for User Story 3

- [ ] T029 [P] [US3] Implement level profile state and progression rules in crates/game_core/src/progression.rs
- [ ] T030 [P] [US3] Define level profile content for the regional dataset in assets/data/levels.v1.json
- [ ] T031 [US3] Integrate level transitions and bucket rotation into the app state in crates/web_app/src/app.rs
- [ ] T032 [US3] Render pre-level labels and transition feedback in crates/web_app/src/render.rs
- [ ] T033 [US3] Persist selected modes and unlocked progress in crates/web_app/src/settings.rs

**Checkpoint**: All user stories are independently functional with progression layered on top of the core loop.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, performance hardening, and delivery preparation across all stories.

- [ ] T034 [P] Add release-ready Trunk and Pages commands in specs/001-mushroom-sorting-game/quickstart.md
- [ ] T035 Add browser performance smoke coverage for frame pacing and feedback latency in tests/integration/performance_smoke.rs
- [ ] T036 [P] Add dataset solvability matrix regression coverage in tests/regression/solvability_matrix.rs
- [ ] T037 Define clean commit boundaries and commit messages for implementation slices in specs/001-mushroom-sorting-game/plan.md
- [ ] T038 Run quickstart validation and capture release readiness notes in specs/001-mushroom-sorting-game/checklists/requirements.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies; start immediately.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational completion.
- **User Story 2 (Phase 4)**: Depends on Foundational completion and can follow US1 independently once shared data scaffolding exists.
- **User Story 3 (Phase 5)**: Depends on Foundational completion and builds most cleanly after US1 and US2 establish the gameplay loop and curated dataset.
- **Polish (Phase 6)**: Depends on the desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: No dependency on other user stories.
- **User Story 2 (P2)**: Reuses the gameplay shell from US1 for live configuration, but remains independently testable once the data pipeline exists.
- **User Story 3 (P3)**: Depends on the gameplay and curated content model established by US1 and US2.

### Within Each User Story

- Tests MUST be written and fail before implementation.
- Core models and rules come before browser integration.
- Dataset and validation tasks come before UI that depends on them.
- Story-specific validation must pass before moving to the next story.

### Parallel Opportunities

- T003 can run in parallel with T001 and T002.
- T005 through T008 can run in parallel after T004 starts the shared schema direction.
- T010 and T011 can run in parallel for US1.
- T012 and T013 can run in parallel for US1.
- T018 and T019 can run in parallel for US2.
- T020 and T021 can run in parallel for US2.
- T027 and T028 can run in parallel for US3.
- T029 and T030 can run in parallel for US3.

---

## Parallel Example: User Story 1

```text
T010 [US1] Add gameplay rule tests in crates/game_core/tests/falling_loop.rs
T011 [US1] Add browser smoke test in tests/integration/game_shell_smoke.rs
T012 [US1] Extract gameplay entities in crates/game_core/src/model.rs
T013 [US1] Implement reducer logic in crates/game_core/src/rules.rs
```

## Parallel Example: User Story 2

```text
T018 [US2] Add solvability validation tests in crates/game_core/tests/configuration_rules.rs
T019 [US2] Add dataset provenance regression checks in tests/regression/mushroom_dataset.rs
T020 [US2] Implement configuration types in crates/game_core/src/configuration.rs
T021 [US2] Parse curated records in data_pipeline/src/records.rs
```

## Parallel Example: User Story 3

```text
T027 [US3] Add progression tests in crates/game_core/tests/level_progression.rs
T028 [US3] Add level transition smoke test in tests/integration/level_transition_smoke.rs
T029 [US3] Implement progression rules in crates/game_core/src/progression.rs
T030 [US3] Define level profiles in assets/data/levels.v1.json
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational.
3. Complete Phase 3: User Story 1.
4. **STOP and VALIDATE**: Run `cargo test -p game_core` and the browser smoke checks before expanding scope.

### Incremental Delivery

1. Setup and Foundational work establish the Rust/WASM delivery path and curated data pipeline.
2. User Story 1 delivers a playable learning loop.
3. User Story 2 adds configurable learning modes backed by curated data.
4. User Story 3 adds progression and level-based bucket rotation.
5. Polish finishes release readiness for GitHub Pages.

### Suggested MVP Scope

Implement through Phase 3 only for the first demonstrable slice.

---

## Notes

- All tasks follow the required checklist format with task id, optional parallel marker, optional story label, and exact file path.
- Total tasks: 38.
- Task count by user story: US1 = 8, US2 = 9, US3 = 7.
- Parallel opportunities identified in Setup, Foundational, and every user story phase.
- Independent test criteria are captured in each user story phase header.