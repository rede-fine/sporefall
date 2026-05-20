# Feature Specification: Sporefall Mushroom Sorting Game

**Feature Branch**: `001-mushroom-sorting-game`

**Created**: 2026-05-19

**Updated**: 2026-05-20

**Status**: In Progress

**Input**: A browser-based educational game where falling mushrooms must be sorted into labeled buckets. Features a two-axis difficulty system (Game Mode × Variety), 28 curated mushroom species with latin names, 4 progressive levels, center-screen feedback animations, a basket collection panel, and educational facts on row clears.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Sort Falling Mushrooms (Priority: P1)

As a player, I want to sort falling mushrooms into the correct bucket lane using keyboard controls so that I can practice recognizing mushroom traits through play.

**Why this priority**: This is the core gameplay loop. Without falling mushrooms, bucket lanes, movement controls, and scoring, no other feature can function.

**Independent Test**: Start a game, use arrow keys to move the mushroom left/right, press spacebar to drop it into a bucket, and verify correct/incorrect feedback is shown.

**Acceptance Scenarios**:

1. **Given** an active game session, **When** a mushroom enters from the top, **Then** it falls downward at a steady speed through the fall zone.
2. **Given** a falling mushroom, **When** the player presses left/right arrow keys, **Then** the mushroom moves to the adjacent bucket lane.
3. **Given** a falling mushroom above the buckets, **When** the player presses spacebar, **Then** the mushroom immediately drops into the current lane.
4. **Given** a mushroom placed in the correct bucket, **When** placement resolves, **Then** a green checkmark animation plays at center screen and points are awarded.
5. **Given** a mushroom placed in the wrong bucket, **When** placement resolves, **Then** a red X animation plays at center screen and the mushroom is ejected to the retry queue.
6. **Given** every bucket lane has at least one resolved mushroom, **When** the row is complete, **Then** a basket bounce animation plays, the row clears, and an educational fact screen is shown.
7. **Given** the fact screen is displayed, **When** the player presses spacebar, **Then** gameplay resumes.

---

### User Story 2 - Configure Game Difficulty (Priority: P2)

As a player, I want to independently choose a Game Mode and Variety setting before starting so that I can match the difficulty to my knowledge level.

**Why this priority**: The two-axis difficulty system is what makes the game approachable for beginners (Normal + Small) and challenging for experts (Expert + Large), directly affecting the game's educational value.

**Independent Test**: Navigate the two-column menu, select different combinations of Game Mode and Variety, start the game, and verify the session reflects both choices.

**Acceptance Scenarios**:

1. **Given** the main menu, **When** it is displayed, **Then** a two-column layout shows Game Mode options on the left and Variety options on the right.
2. **Given** the menu, **When** the player presses left/right arrows, **Then** focus switches between the Game Mode and Variety columns.
3. **Given** the Game Mode column is active, **When** the player selects Normal, **Then** falling mushrooms show photo + common name + latin name (centered below image).
4. **Given** the Game Mode column is active, **When** the player selects Tricky, **Then** falling mushrooms show photo only with no names.
5. **Given** the Game Mode column is active, **When** the player selects Expert, **Then** falling mushrooms show a generic emoji + latin name only.
6. **Given** the Variety column is active, **When** the player selects Small/Medium/Large, **Then** the session uses 12/20/28 mushroom species respectively.
7. **Given** any valid selection, **When** the player presses Enter or Spacebar, **Then** the game starts with those settings.

---

### User Story 3 - Progress Through 4 Levels (Priority: P3)

As a player, I want to advance through 4 levels that each use a different sorting category so that I learn multiple classification systems in one session.

**Why this priority**: Level progression provides structured learning and replay value, teaching players multiple ways to classify the same species.

**Independent Test**: Complete all mushrooms in a level, verify the Level Complete screen appears, press spacebar to advance, and confirm the next level uses a different category system.

**Acceptance Scenarios**:

1. **Given** a new game, **When** the first level starts, **Then** the active sorting category is Cap Color with lanes: Red/Orange, Brown/Tan, White/Cream, Yellow/Gold.
2. **Given** Level 1 is complete, **When** Level 2 starts, **Then** the category switches to Culinary Type with lanes: Choice Edible, Medicinal, Toxic, Inedible.
3. **Given** Level 2 is complete, **When** Level 3 starts, **Then** the category switches to Ecological Role with lanes: Mycorrhizal, Saprotrophic, Parasitic, Endophytic.
4. **Given** Level 3 is complete, **When** Level 4 starts, **Then** the category switches to Peak Season with lanes: Spring, Summer, Autumn, Winter.
5. **Given** all mushrooms in the current variety have been correctly sorted for a level, **When** the level ends, **Then** a Level Complete screen is shown with a collection review.
6. **Given** the Level Complete screen, **When** the player presses spacebar, **Then** the next level begins (or Game Over after Level 4).
7. **Given** a mushroom is correctly sorted in the current level, **When** the next mushroom is selected, **Then** the correctly sorted mushroom will not reappear in that level.

---

### User Story 4 - View Basket Collection (Priority: P4)

As a player, I want to see a panel on the right side showing all mushrooms I've collected in baskets so that I can track my progress and review what I've learned.

**Why this priority**: The collection panel provides ongoing visual reward and reinforces learning by showing accumulated progress without interrupting gameplay.

**Independent Test**: Clear a row during gameplay and verify the collected mushrooms appear as thumbnails in the right-side panel with their names.

**Acceptance Scenarios**:

1. **Given** an active game session, **When** the game area is rendered, **Then** a basket collection panel is displayed on the right side of the screen.
2. **Given** a row has been cleared, **When** the mushrooms are collected, **Then** they appear as thumbnail images in a grid within the collection panel.
3. **Given** many mushrooms are collected, **When** the panel fills up, **Then** overflow is indicated with a "+N more" count.
4. **Given** the collection panel, **When** any mushroom is displayed, **Then** a short version of its name appears centered below its thumbnail.

---

### User Story 5 - Learn from Educational Facts (Priority: P5)

As a player, I want to see interesting educational facts about mushrooms when I clear a basket so that I learn beyond just classification.

**Why this priority**: Facts transform row clears from pure mechanics into learning moments, fulfilling the educational mission of the game.

**Independent Test**: Clear a row containing specific mushroom combinations and verify context-appropriate facts are shown.

**Acceptance Scenarios**:

1. **Given** a basket row is cleared, **When** the basket bounce animation completes, **Then** an educational fact screen appears with the cleared mushrooms shown.
2. **Given** the fact screen, **When** the basket contains a Death Cap, **Then** a danger warning fact specific to that species is shown.
3. **Given** the fact screen, **When** the basket contains medicinal mushrooms, **Then** a fact about their traditional/modern medicinal use is shown.
4. **Given** the fact screen, **When** the player presses spacebar, **Then** gameplay resumes with the next falling mushroom.

### Edge Cases

- What happens when a mushroom reaches the bottom without player input? It auto-drops into the current lane and resolves as a normal placement.
- What happens when a mushroom is placed incorrectly? It is ejected to a retry queue and will re-enter play from the top.
- What happens when all mushrooms in the variety set have been correctly sorted for a level? The level transitions to Level Complete phase.
- How does the system handle the last level (Level 4) completion? The game transitions to Game Over, showing final score and returning to menu on spacebar.
- What happens when the collection panel runs out of vertical space? An overflow count "+N more" is shown at the bottom of the panel.
- Can the player pause during gameplay? Yes, pressing ESC shows a pause overlay; pressing ESC again or Enter resumes play.

## Requirements *(mandatory)*

### Functional Requirements

#### Core Gameplay

- **FR-001**: The system MUST present one mushroom at a time entering from the top of the fall zone and traveling downward at a configurable speed.
- **FR-002**: The player MUST be able to move the falling mushroom left/right between bucket lanes using arrow keys.
- **FR-003**: The player MUST be able to hard-drop the mushroom into the current lane by pressing spacebar.
- **FR-004**: The system MUST display 4 labeled bucket lanes at the bottom of the play area for the active sorting category.
- **FR-005**: The system MUST evaluate each placed mushroom against the correct lane for the active category and provide immediate visual feedback.
- **FR-006**: The system MUST show a green checkmark center-screen animation when a mushroom is placed correctly.
- **FR-007**: The system MUST show a red X center-screen animation when a mushroom is placed incorrectly.
- **FR-008**: The system MUST show a basket bounce center-screen animation when a complete row is cleared.
- **FR-009**: When a mushroom is placed incorrectly, it MUST be ejected to a retry queue and re-enter play from the top.
- **FR-010**: When every bucket lane has at least one resolved entry, the bottom row MUST clear and award bonus points.
- **FR-011**: The system MUST award points for each correct placement and additional bonus points for row clears.

#### No-Repeat Mechanic

- **FR-012**: Once a mushroom is correctly sorted in a level, it MUST NOT appear again in that level.
- **FR-013**: If all mushrooms in the selected variety have been correctly sorted for the current level, the system MUST transition to the Level Complete phase.

#### Two-Axis Difficulty

- **FR-014**: The Game Mode axis MUST control what identifying information is displayed for each falling mushroom:
  - Normal: mushroom photo + common name + latin name (centered below image)
  - Tricky: mushroom photo only (no names shown)
  - Expert: generic mushroom emoji + latin name only
- **FR-015**: The Variety axis MUST control how many mushroom species are available in the session:
  - Small: 12 species
  - Medium: 20 species
  - Large: 28 species (full catalog)
- **FR-016**: Game Mode and Variety MUST be selectable independently via a two-column menu.

#### Level Progression

- **FR-017**: The game MUST provide exactly 4 levels in fixed order: Cap Color → Culinary Type → Ecological Role → Peak Season.
- **FR-018**: Each level MUST use 4 mutually exclusive bucket lanes corresponding to its category system:
  - Level 1 (Cap Color): Red/Orange, Brown/Tan, White/Cream, Yellow/Gold
  - Level 2 (Culinary Type): Choice Edible, Medicinal, Toxic, Inedible
  - Level 3 (Ecological Role): Mycorrhizal, Saprotrophic, Parasitic, Endophytic
  - Level 4 (Peak Season): Spring, Summer, Autumn, Winter
- **FR-019**: Upon completing all mushrooms in a level, the system MUST show a Level Complete screen with a collection review of all mushrooms sorted.
- **FR-020**: The player MUST advance from the Level Complete screen by pressing spacebar.
- **FR-021**: After completing all 4 levels, the system MUST show a Game Over screen with the final score.

#### Mushroom Catalog

- **FR-022**: The game MUST include a curated catalog of exactly 28 mushroom species, each with a common name, latin (scientific) name, image, and correct lane assignments for all 4 category systems.
- **FR-023**: The catalog MUST be structured so the first 12 entries form the Small set, the first 20 form the Medium set, and all 28 form the Large set.
- **FR-024**: Mushroom names MUST be displayed centered below their images during the fall (when the Game Mode includes names).

#### Basket Collection Panel

- **FR-025**: The system MUST display a basket collection panel on the right side of the game area showing all mushrooms collected across basket clears.
- **FR-026**: Collected mushrooms MUST be shown as a thumbnail grid with abbreviated names centered below each thumbnail.
- **FR-027**: The panel MUST handle overflow gracefully by showing a "+N more" indicator when vertical space is exhausted.

#### Educational Facts

- **FR-028**: When a row is cleared, the system MUST display an educational fact screen after the basket bounce animation completes.
- **FR-029**: Facts MUST be context-sensitive, reflecting the specific mushrooms in the cleared basket (e.g., danger warnings for toxic species, culinary tips for edible combos, medicinal information for medicinal species).
- **FR-030**: The player MUST dismiss the fact screen by pressing spacebar to resume play.

#### Menu & Configuration

- **FR-031**: The main menu MUST show a two-column layout: Game Mode selection on the left, Variety selection on the right.
- **FR-032**: Menu navigation MUST use arrow keys (left/right switches columns, up/down selects within a column) and Enter/Space to start.
- **FR-033**: The menu MUST display the level progression sequence ("4 Levels: Cap Color → Culinary Type → Ecological Role → Peak Season").

#### Controls & Accessibility

- **FR-034**: The player MUST be able to pause the game by pressing ESC and resume by pressing ESC or Enter.
- **FR-035**: The game MUST display control hints during gameplay (arrow keys, spacebar, ESC).
- **FR-036**: The game MUST show the current score, basket count, and sorted progress (X/Y mushrooms) in the header area.

#### Deployment & Data

- **FR-037**: The game MUST be deployable as a static website without server-side runtime.
- **FR-038**: The game MUST use real mushroom photographs sourced from public-domain or permissively-licensed collections.
- **FR-039**: The game MUST retain provenance metadata (image license, attribution, source URL) for each mushroom entry.
- **FR-040**: The deployment at https://rede-fine.github.io/sporefall/ MUST reflect the latest release build.

### Key Entities

- **Mushroom Catalog Entry**: A species in the game's database containing: record ID, common name, latin (scientific) name, image asset key, and a target lane index for each of the 4 category systems (ecology, color, season, function).
- **Falling Mushroom**: An active game piece with ID, display name, latin name, target lane for the current category, and image key. Travels from top to bottom of the fall zone.
- **Category Mode**: One of 4 sorting systems (Cap Color, Culinary Type, Ecological Role, Peak Season) with 4 mutually exclusive lane labels each.
- **Game Mode (Difficulty)**: Controls what info is shown — Normal (full info), Tricky (photo only), Expert (emoji + latin name).
- **Variety**: Controls species count — Small (12), Medium (20), Large (28).
- **Basket Collection**: Accumulated mushrooms from all row clears in the session, displayed in the right-side panel.
- **Center Animation**: Visual feedback effect (green checkmark, red X, or basket bounce) played at the center of the game area.
- **Level Profile**: A fixed progression step (1-4) that determines which Category Mode is active.
- **Placement Result**: The outcome of dropping a mushroom, including whether it was correct, points awarded, whether a row was cleared, and which mushrooms were collected.

## Future Features *(out of scope for current release)*

- **iNaturalist Integration**: User provides their iNaturalist account and a time window; the game uses their actual observations as mushroom entries for a personalized learning experience.
- **Mobile-Friendly Touch Controls**: Tap a lane to sort the falling mushroom directly into it, optimized for touch devices.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 90% of first-time players can navigate the menu, select settings, and start a game within 30 seconds without outside help.
- **SC-002**: 85% of players can correctly sort at least one mushroom in Level 1 (Cap Color) during their first session, regardless of Game Mode.
- **SC-003**: Players can complete a full level (all mushrooms in their selected variety sorted) within 10 minutes on Normal/Small settings.
- **SC-004**: All visual feedback (correct/wrong/basket animations) appears within 100ms of the triggering action and completes within 500ms.
- **SC-005**: 80% of playtesters can explain what at least one educational fact taught them about mushrooms after a session.
- **SC-006**: The two-axis difficulty system provides perceivable difficulty differences: Expert mode completion rates are at least 30% lower than Normal mode for the same Variety.
- **SC-007**: The no-repeat mechanic ensures every mushroom in the variety set is sorted exactly once per level, providing complete catalog coverage per level completion.

## Assumptions

- The game is single-player with no account system, leaderboard, or persistent progression across sessions.
- The 28-species catalog is pre-curated and hardcoded; runtime data loading is not required for v1.
- All mushroom images are sourced from Wikimedia Commons or equivalent permissively-licensed sources.
- The initial content scope is Central European mushroom species for curation tractability.
- The game uses keyboard controls exclusively for v1; touch/mobile controls are a future enhancement.
- The game runs at a fixed canvas resolution (960×540) suitable for desktop browsers.
- Fall speed is fixed at 0.35 (fraction of height per second) and is not player-configurable in v1.
- The retry queue (wrong placements re-entering) has no limit; incorrectly placed mushrooms keep returning until correctly sorted.
- Educational facts are pattern-matched from mushroom combinations and hardcoded in the game logic.