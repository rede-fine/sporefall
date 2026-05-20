# Feature Specification: Sporefall Mushroom Sorting Game

**Feature Branch**: `[001-mushroom-sorting-game]`

**Created**: 2026-05-19

**Status**: Draft

**Input**: User description: "I want to build a webbrowser based game called sporefall for people who want to learn how to classify mussrooms. The rough idea is that a mushroom enters the screen from the top and travels down, like in tetris. in the bottom there are different buckets where you have to sort them in, like Ecological Classifications (Saprotrophic, Mycorrhizal, Parasitic, Endophytic), Biological & Taxonomic Grouping, Functional / Culinary Types, or season when to find it best (user should have option to choose which buckets are shown in a game and it could also vary by levels). also include one catogory that is very obvious from the image, like color, so also people with no mushroom knowledge can play it. the entering mushroom can be represented by common name, latin name, and/or a picture (user should be able to select this). the user can use arrow keys to move the mushroom in the correct bucket and press space to get it to the bottom, as in tetris. when each bucket has an entry, the bottom row disappears and you get Points."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Sort Falling Mushrooms (Priority: P1)

As a player, I want to sort falling mushrooms into the correct bottom buckets using keyboard controls so that I can practice recognizing mushroom traits through play.

**Why this priority**: This is the core learning loop. Without falling mushrooms, buckets, and scoring for completed rows, the feature does not deliver either gameplay or educational value.

**Independent Test**: Can be fully tested by starting a game, moving falling mushrooms left and right with the keyboard, dropping them into buckets, and confirming that correctly filled rows clear and award points.

**Acceptance Scenarios**:

1. **Given** a game session with active buckets, **When** a mushroom begins falling, **Then** the player can move it horizontally between bucket lanes before it lands.
2. **Given** a falling mushroom above the buckets, **When** the player presses the drop key, **Then** the mushroom immediately settles into the current bucket lane.
3. **Given** each active bucket lane in the bottom row contains one mushroom entry, **When** the final open lane is filled, **Then** the row clears and the player receives points.
4. **Given** a mushroom is placed into an incorrect bucket, **When** the placement is resolved, **Then** the player receives clear corrective feedback before play continues.

---

### User Story 2 - Configure Learning Mode (Priority: P2)

As a player, I want to choose which bucket categories and mushroom clues are shown so that I can match the game difficulty to my knowledge level.

**Why this priority**: The game is meant for both beginners and learners with domain knowledge. Configurable bucket sets and clue types make the experience useful across different skill levels.

**Independent Test**: Can be fully tested by selecting a subset of bucket categories and clue types before starting a session, then confirming gameplay uses only those selected settings.

**Acceptance Scenarios**:

1. **Given** the game setup screen, **When** the player selects one or more bucket categories, **Then** the session starts with only those categories shown.
2. **Given** the game setup screen, **When** the player enables common name, Latin name, image, or any combination of them, **Then** each falling mushroom uses the chosen clue types during play.
3. **Given** a beginner-friendly session, **When** the player includes an obvious visual category such as color, **Then** at least one bucket set is solvable from the visible clue alone.

---

### User Story 3 - Learn Through Progression (Priority: P3)

As a returning player, I want levels to vary the active bucket sets and challenge so that the game stays engaging while gradually expanding what I learn.

**Why this priority**: Progression increases replay value and supports structured learning, but it depends on the core gameplay and configuration experience already working.

**Independent Test**: Can be fully tested by completing early rounds, advancing to later levels, and confirming the active buckets or challenge mix changes while keeping controls and scoring behavior consistent.

**Acceptance Scenarios**:

1. **Given** the player advances through levels, **When** a new level begins, **Then** the active bucket set may change according to that level's configuration.
2. **Given** the player has learned one category set, **When** a later level introduces a different category set, **Then** the game clearly shows the new bucket labels before play resumes.
3. **Given** multiple levels use different bucket combinations, **When** the player moves between levels, **Then** the same keyboard controls and row-clearing rules remain consistent.

### Edge Cases

- What happens when the player starts a session without selecting any bucket category? The game must block session start and explain that at least one category set is required.
- What happens when the player selects clue types that do not support a chosen category? The game must only offer playable combinations and explain unavailable options.
- How does the system handle repeated mushrooms appearing in the same round? Repeats are allowed, but each appearance must still be scored and evaluated independently.
- What happens when a falling mushroom reaches the bottom without player input? It must settle into the lane directly beneath its final position and resolve as a normal placement.
- How does the system handle a level-specific bucket change mid-session? The new bucket labels must be shown before the next falling mushroom appears.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide a single-player browser game experience centered on sorting falling mushrooms into labeled bucket lanes.
- **FR-002**: The system MUST present one mushroom at a time entering from the top play area and moving downward until it is placed.
- **FR-003**: The player MUST be able to move the falling mushroom horizontally with the keyboard and trigger an immediate drop with the space bar.
- **FR-004**: The system MUST show labeled bucket lanes at the bottom of the play area for the active category set in the current session or level.
- **FR-005**: The system MUST allow the player to choose which bucket category sets are included in a game session before gameplay begins.
- **FR-006**: The system MUST support category sets that cover ecological classifications, biological or taxonomic grouping, functional or culinary types, seasonality, and at least one visually obvious category such as color.
- **FR-007**: The system MUST allow the player to choose whether each falling mushroom is shown by common name, Latin name, image, or any combination of those clue types.
- **FR-008**: The system MUST ensure that every configured session includes at least one playable bucket set and at least one visible clue type.
- **FR-009**: The system MUST evaluate each placed mushroom against the active bucket lane and provide immediate feedback indicating whether the placement was correct.
- **FR-010**: The system MUST award points whenever the player completes a bottom row by filling every active bucket lane with one resolved mushroom entry.
- **FR-011**: The system MUST clear a completed bottom row after scoring and continue the session with the next falling mushroom.
- **FR-012**: The system MUST support level-based variation so that active bucket sets can differ between levels while preserving the same core controls and scoring rules.
- **FR-013**: The system MUST show the active bucket labels clearly before gameplay starts and whenever a level changes the bucket set.
- **FR-014**: The system MUST prevent the player from starting a game configuration that would be unsolvable from the selected clues and buckets.
- **FR-015**: The system MUST track and display the player's current score during active gameplay.
- **FR-016**: The first release MUST be deliverable as a static public website without requiring a server-side runtime during play.
- **FR-017**: The system MUST use a curated mushroom dataset where every playable entry includes enough verified metadata to support all configured bucket systems it appears in.
- **FR-018**: The system MUST retain provenance for each curated mushroom entry so taxonomy, ecological labels, seasonal labels, and image reuse can be audited.
- **FR-019**: The system MUST only ship public-facing images whose reuse terms are compatible with a publicly hosted educational browser game.
- **FR-020**: The system MUST support a curated regional content scope for the initial release so the dataset remains reviewable and the bucket combinations remain reliable.
- **FR-021**: The system MUST animate the falling mushroom downward at a configurable speed to create time pressure for the sorting decision.
- **FR-022**: The system MUST collect correctly classified mushrooms in a visible reward basket rather than simply removing them from the play area.
- **FR-023**: The system MUST present bucket categories as coherent, mutually exclusive sets (e.g., all ecology labels OR all color labels in one session) that the player selects before starting.
- **FR-024**: The system MUST allow multiple mushrooms to land in the same bucket lane; stacking is permitted and the row clears only when every lane has at least one entry.
- **FR-025**: The system MUST display a representative image of each falling mushroom alongside or instead of its text clue, depending on the player's clue mode selection.
- **FR-026**: The bucket zone at the bottom of the play area MUST occupy a small fraction of the vertical space so that the majority of the fall distance gives the player time to decide.
- **FR-027**: When a mushroom is placed in the wrong bucket, it MUST NOT remain in that bucket; instead it MUST be ejected to the side and re-enter play from the top as a retry.
- **FR-028**: The system MUST provide a clearly accessible way for the player to stop or pause the current game session at any time.
- **FR-029**: The system MUST use real mushroom photographs or high-quality illustrations sourced from public-domain or permissively-licensed collections for each playable species.
- **FR-030**: The game deployment at https://rede-fine.github.io/sporefall/ MUST reflect the latest release build.

### Key Entities *(include if feature involves data)*

- **Mushroom Card**: A playable mushroom prompt containing its identity clues, accepted bucket labels for the active categories, and level-appropriate learning metadata.
- **Bucket Set**: A named group of bucket lanes representing one classification scheme, such as ecological role, color, season, or culinary type.
- **Game Session**: A single run of the game containing the selected clue types, selected bucket sets, current score, and current level state.
- **Level Profile**: A progression step that defines which bucket sets are active and how the session challenge changes over time.
- **Placement Result**: The outcome of dropping one mushroom into one lane, including whether it was correct, the feedback shown, and whether it contributed to clearing the row.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 90% of test players can start a configured game session in under 60 seconds without outside help.
- **SC-002**: 85% of first-time players can correctly place at least one mushroom in a visually obvious category during their first session.
- **SC-003**: 80% of players can complete at least one scored row in a four-bucket session within 5 minutes.
- **SC-004**: After a completed row, the updated score and cleared row state are visible to the player within 1 second.
- **SC-005**: In moderated playtests, at least 75% of players report that the difference between bucket categories and clue settings is understandable before their first round starts.

## Assumptions

- The first release is a single-player browser experience with no account system, multiplayer mode, or persistent progression across devices.
- The game uses a curated mushroom dataset where each mushroom already has valid labels for the supported bucket categories.
- A session may include one or more bucket sets, but only combinations that remain solvable from the selected clue types are offered to the player.
- The same keyboard controls are used throughout the game to keep the experience consistent across levels.
- Educational feedback is concise and shown in the moment of placement rather than through a separate study mode in this feature.
- The initial release is hosted as a static public website, which keeps deployment simple and rules out backend-dependent features in scope.
- The initial content set is regional rather than global so that scientific curation, licensing review, and playability checks stay tractable.