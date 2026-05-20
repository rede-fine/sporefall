# Tasks: Sporefall Mushroom Sorting Game

**Input**: Design documents from `/specs/001-mushroom-sorting-game/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Tests**: Tests are required for gameplay rules, browser-shell behavior, and regressions that affect player-visible behavior.

**Organization**: Tasks are grouped by user story to preserve independent delivery and testing.

**Status**: Core implementation complete. All 5 user stories delivered. 7 unit tests passing. Remaining work is asset sourcing and future features.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g. `[US1]`, `[US2]`, `[US3]`)
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure) ✅

**Purpose**: Shared Rust/WASM tooling and Trunk deployment configuration.

- [x] T001 Configure Trunk application build settings in Trunk.toml
- [x] T002 Add GitHub Pages deployment workflow in .github/workflows/deploy-pages.yml
- [x] T003 [P] Add local cargo aliases and WASM target defaults in .cargo/config.toml

---

## Phase 2: Foundational (Blocking Prerequisites) ✅

**Purpose**: Core data and application foundations that block all user stories.

- [x] T004 Define the curated mushroom dataset schema in assets/data/mushrooms.schema.json
- [x] T005 [P] Scaffold the offline data pipeline crate in data_pipeline/Cargo.toml
- [x] T006 [P] Implement source-record normalization entrypoint in data_pipeline/src/main.rs
- [x] T007 [P] Create shared web application state container in crates/web_app/src/app.rs
- [x] T008 [P] Add browser settings persistence model in crates/web_app/src/settings.rs
- [x] T009 Create image attribution manifest structure in assets/data/image_attributions.json

**Checkpoint**: ✅ Rust workspace, build pipeline, and curated data foundations ready.

---

## Phase 3: User Story 1 – Sort Falling Mushrooms (Priority: P1) ✅

**Goal**: Core falling-mushroom loop with keyboard control, placement feedback, row clearing, retry queue, and score updates.

**Independent Test**: Start a local session, move a falling mushroom between lanes with arrow keys, press space to drop it, verify correct/incorrect center animations, row clearing with basket bounce, retry queue re-entry, and score changes.

### Tests for User Story 1

- [x] T010 [P] [US1] Add gameplay rule tests (spawn, movement, hard drop, row clearing, retry queue, scoring) in crates/game_core/tests/falling_loop.rs
- [x] T011 [P] [US1] Add browser smoke test for canvas rendering and keyboard loop in tests/integration/game_shell_smoke.rs

### Implementation for User Story 1

- [x] T012 [P] [US1] Define core gameplay entities (FallingMushroom, GameConfig, PlacementFeedback, BucketSet) in crates/game_core/src/model.rs
- [x] T013 [P] [US1] Implement game state machine (lanes, basket, retry queue, score, row clearing) in crates/game_core/src/rules.rs
- [x] T014 [US1] Implement keyboard input mapping (arrows, space, ESC) in crates/web_app/src/input.rs
- [x] T015 [US1] Render lanes, falling mushrooms, center-screen animations (correct/wrong/basket), and placement feedback in crates/web_app/src/render.rs
- [x] T016 [US1] Connect the requestAnimationFrame loop to core update logic in crates/web_app/src/lib.rs
- [x] T017 [US1] Style the in-game score, basket count, and sorted progress header in styles.css

**Checkpoint**: ✅ Core gameplay loop complete with all feedback animations and retry mechanic.

---

## Phase 4: User Story 2 – Configure Game Difficulty (Priority: P2) ✅

**Goal**: Two-axis difficulty system (Game Mode × Variety) with two-column menu selection.

**Independent Test**: Navigate the two-column menu, select different Game Mode and Variety combinations, start the game, and verify the session reflects both choices (species count, display mode).

### Implementation for User Story 2

- [x] T018 [US2] Implement Game Mode enum (Normal/Tricky/Expert) and Variety enum (Small/Medium/Large) in crates/web_app/src/catalog.rs
- [x] T019 [US2] Build two-column menu renderer (Game Mode left, Variety right) with arrow-key navigation in crates/web_app/src/render.rs
- [x] T020 [US2] Wire menu selection into GamePhase transitions and session configuration in crates/web_app/src/app.rs
- [x] T021 [US2] Implement pick_mushroom with variety-filtered no-repeat selection in crates/web_app/src/catalog.rs

**Checkpoint**: ✅ Two-axis difficulty fully functional with 12/20/28 species filtering.

---

## Phase 5: User Story 3 – Progress Through 4 Levels (Priority: P3) ✅

**Goal**: 4-level progression (Cap Color → Culinary Type → Ecological Role → Peak Season) with Level Complete screens.

**Independent Test**: Complete all mushrooms in a level, verify Level Complete screen appears with collection review, press spacebar to advance, confirm next level uses a different category system.

### Implementation for User Story 3

- [x] T022 [US3] Implement CategoryMode enum with 4 sorting systems and bucket lane definitions in crates/web_app/src/catalog.rs
- [x] T023 [US3] Add level progression logic (sorted_this_level HashSet, level transition triggers) in crates/web_app/src/app.rs
- [x] T024 [US3] Render Level Complete screen with collection review and Game Over screen in crates/web_app/src/render.rs

**Checkpoint**: ✅ Full 4-level progression with no-repeat mechanic per level.

---

## Phase 6: User Story 4 – View Basket Collection (Priority: P4) ✅

**Goal**: Right-side panel showing collected mushrooms as thumbnails with names and overflow handling.

**Independent Test**: Clear a row during gameplay and verify collected mushrooms appear as thumbnails in the right-side panel with abbreviated names.

### Implementation for User Story 4

- [x] T025 [US4] Render basket collection panel with thumbnail grid on the right side of the game area in crates/web_app/src/render.rs
- [x] T026 [US4] Track collected mushrooms across basket clears in app state in crates/web_app/src/app.rs
- [x] T027 [US4] Handle panel overflow with "+N more" indicator in crates/web_app/src/render.rs

**Checkpoint**: ✅ Collection panel displays accumulated basket contents.

---

## Phase 7: User Story 5 – Learn from Educational Facts (Priority: P5) ✅

**Goal**: Context-sensitive educational fact screens after basket row clears.

**Independent Test**: Clear a row containing specific mushrooms and verify an appropriate fact screen appears before gameplay resumes.

### Implementation for User Story 5

- [x] T028 [US5] Implement context-sensitive fact generation (danger warnings, medicinal info, culinary tips) in crates/web_app/src/facts.rs
- [x] T029 [US5] Render fact screen with cleared mushroom display and spacebar-to-resume in crates/web_app/src/render.rs
- [x] T030 [US5] Integrate fact phase into GamePhase transitions after basket bounce animation in crates/web_app/src/app.rs

**Checkpoint**: ✅ Educational facts display after every row clear.

---

## Phase 8: Infrastructure & Assets ✅

**Purpose**: Image preloading, catalog population, and deployment configuration.

- [x] T031 [P] Build async image preloading system for all 28 species in crates/web_app/src/images.rs
- [x] T032 [P] Populate full 28-species catalog with correct lane assignments for all 4 category systems in crates/web_app/src/catalog.rs
- [x] T033 [P] Populate curated mushroom dataset (28 species) in assets/data/mushrooms.v1.json
- [x] T034 Configure Trunk build for GitHub Pages deployment in Trunk.toml and index.html

**Checkpoint**: ✅ 28-species catalog with image preloading and Trunk deployment ready.

---

## Phase 9: Remaining Work (Not Started)

**Purpose**: Asset sourcing, future features, and visual polish.

### Asset Sourcing

- [ ] T035 [P] Source real CC0/public-domain photographs for 16 new mushroom species (currently placeholder copies of chanterelle.jpg) in assets/images/mushrooms/
- [ ] T036 [P] Update image attribution entries for all newly sourced photographs in assets/data/image_attributions.json

### Future Feature: iNaturalist Integration

- [ ] T037 [P] Design iNaturalist observation import flow (user provides account + time window) in specs/001-mushroom-sorting-game/spec.md
- [ ] T038 Implement iNaturalist API client for downloading user observations in data_pipeline/src/
- [ ] T039 Map iNaturalist observations to game catalog format with image download in data_pipeline/src/export.rs
- [ ] T040 Add UI for entering iNaturalist account and time window selection in crates/web_app/src/

### Future Feature: Mobile Touch Controls

- [ ] T041 [P] Design tap-lane-to-sort interaction model for touch devices in specs/001-mushroom-sorting-game/spec.md
- [ ] T042 Implement touch event handlers (tap lane to move + drop) in crates/web_app/src/input.rs
- [ ] T043 Add responsive layout adjustments for mobile screen sizes in crates/web_app/src/render.rs and styles.css

### Visual Polish

- [ ] T044 [P] Tune center-screen animation timing and easing curves in crates/web_app/src/render.rs
- [ ] T045 [P] Implement responsive canvas sizing for different viewport dimensions in crates/web_app/src/render.rs
- [ ] T046 Add smooth transitions between game phases (fade/slide) in crates/web_app/src/render.rs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: ✅ Complete.
- **Foundational (Phase 2)**: ✅ Complete.
- **User Story 1 (Phase 3)**: ✅ Complete.
- **User Story 2 (Phase 4)**: ✅ Complete.
- **User Story 3 (Phase 5)**: ✅ Complete.
- **User Story 4 (Phase 6)**: ✅ Complete.
- **User Story 5 (Phase 7)**: ✅ Complete.
- **Infrastructure (Phase 8)**: ✅ Complete.
- **Remaining (Phase 9)**: Independent tracks; asset sourcing has no code dependencies; future features require design first.

### Remaining Work Independence

- **Asset sourcing (T035–T036)**: Can proceed immediately; no code changes needed, only replacing placeholder images.
- **iNaturalist integration (T037–T040)**: Requires design spec first (T037), then sequential implementation.
- **Mobile touch controls (T041–T043)**: Requires design spec first (T041), then input + render changes.
- **Visual polish (T044–T046)**: All parallelizable; no dependencies on other remaining work.

### Parallel Opportunities (Remaining)

- T035 and T036 can run in parallel (different files).
- T037, T041, T044, T045 can all run in parallel (independent design/render tracks).
- T044, T045, T046 are all parallelizable within visual polish.

---

## Summary

| Category | Tasks | Status |
|----------|-------|--------|
| Setup & Foundational | T001–T009 | ✅ Done (9 tasks) |
| User Story 1 – Core Gameplay | T010–T017 | ✅ Done (8 tasks) |
| User Story 2 – Difficulty Config | T018–T021 | ✅ Done (4 tasks) |
| User Story 3 – Level Progression | T022–T024 | ✅ Done (3 tasks) |
| User Story 4 – Basket Collection | T025–T027 | ✅ Done (3 tasks) |
| User Story 5 – Educational Facts | T028–T030 | ✅ Done (3 tasks) |
| Infrastructure & Assets | T031–T034 | ✅ Done (4 tasks) |
| Asset Sourcing | T035–T036 | ⬜ Not started (2 tasks) |
| iNaturalist Integration | T037–T040 | ⬜ Not started (4 tasks) |
| Mobile Touch Controls | T041–T043 | ⬜ Not started (3 tasks) |
| Visual Polish | T044–T046 | ⬜ Not started (3 tasks) |
| **Total** | **T001–T046** | **34 done, 12 remaining** |