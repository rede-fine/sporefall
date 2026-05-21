# Sporefall — Feature Roadmap & Ideas

**Updated**: 2026-05-21

This document captures all planned improvements, in-progress work, and future ideas discussed during development. It serves as the single source of truth for what's been done and what remains.

---

## Completed (This Session)

### iNaturalist Import Overhaul
- [x] Import accepts ALL fungi species from iNaturalist (not just catalog matches)
- [x] Extended species database (`species_db.rs`) with 100+ species and 50+ genus-level fallbacks for game attribute assignment
- [x] iNaturalist appears as 4th Variety option in menu when data is imported (auto-selected)
- [x] Species selection/deselection UI with checkboxes, thumbnails, and "Deselect All" toggle
- [x] Photos from iNaturalist load via CORS (prefer CC-licensed taxon `default_photo`)
- [x] Import button no longer auto-starts game; stores data and returns to menu
- [x] Date range defaults: FROM empty (all time), TO today
- [x] Status message updated for clarity

### UI & Alignment Fixes
- [x] Pause overlay: 88% opacity, centered "Paused" text + subtitle, buttons vertically centered (no longer overlapping buckets)
- [x] Button text uses `textAlign("center")` for proper centering
- [x] Game header shows `[Normal / iNaturalist]` (no redundant label)
- [x] Feedback message positioned above "4 levels" text with distinct gold color
- [x] Menu cards resize to fit 4 variety options when iNat is available

### Gameplay
- [x] Starting fall speed reduced from 0.35 → 0.18
- [x] Gradual speed increase: +0.025 per basket cleared (caps at 0.50)
- [x] Adaptive difficulty scaling based on recent accuracy
- [x] 🍄 emoji in page title and SVG favicon

### Educational Content
- [x] Facts database expanded from 4 generic + 10 specific → 16 generic + 25 species-specific facts
- [x] All facts now include `[Source: ...]` citations (journals, books, organizations)
- [x] Facts use deterministic hash-based selection for variety without randomness

### Review & Collection UX
- [x] Game over screen shows session accuracy and most-struggled species stats
- [x] Imported observation species can cycle through actual observation gallery photos during gameplay
- [x] Collection thumbnails on review screens open a species detail card overlay

### Image Assets
- [x] Audited duplicate asset folders and removed redundant `assets/images/mushrooms/`
- [x] Verified bundled built-in images in `assets/mushroom-images/` are distinct photographs, not a single repeated placeholder file

---

## In Progress / Partially Complete

### Species Database Expansion
- [ ] Verify all 28 built-in catalog entries have correct attributes (ecology, color, season, function)
- [ ] Cross-reference targets against authoritative sources (MycoBank, Index Fungorum)
- [ ] Consider expanding built-in Large set beyond 28 species (40? 50?)
- [ ] Add more species to `species_db.rs` extended database (currently ~100, goal: 300+)

### Image Assets
- [ ] Fill `assets/data/image_attributions.json` with complete bundled-photo licensing metadata

---

## Not Yet Implemented (Future Ideas)

### Gameplay Enhancements
- [ ] **Streak bonus**: Extra points for consecutive correct placements
- [ ] **Time-limited mode**: Race against clock instead of free-fall
- [ ] **Sound effects**: Correct/wrong/basket-clear audio feedback
- [ ] **Power-ups**: Slow-motion, lane preview, second chance

### iNaturalist Integration
- [ ] **Observation details**: Show date/location of user's observation when sorting their species
- [ ] **Multi-user comparison**: Import multiple users' observations and play combined set
- [ ] **Auto-refresh**: Detect new observations and offer to update imported set

### Educational Features
- [ ] **Learning mode**: Slower pace with hints about which lane is correct
- [ ] **Quiz mode**: After game, quiz player on facts learned during session
- [ ] **Spore print guide**: Visual reference for mushroom identification
- [ ] **Habitat info**: Show where each species grows (forest type, substrate, altitude)
- [ ] **Seasonal foraging calendar**: Interactive calendar showing what's in season

### Social & Competitive
- [ ] **Online leaderboard**: Global scores (currently local SQLite only)
- [ ] **Daily challenge**: Same species set for all players each day
- [ ] **Achievement badges**: Collect all edibles, identify all toxic, etc.
- [ ] **Share results**: Generate shareable image of game results

### Technical Improvements
- [ ] **Offline support**: Service worker for PWA offline play
- [ ] **Accessibility**: Screen reader support for game state
- [ ] **Mobile-first redesign**: Touch gestures (swipe to lane, tap to drop)
- [ ] **Localization**: Support for multiple languages (species names, UI text)
- [ ] **Analytics**: Track which species are most commonly mis-sorted (anonymized)
- [ ] **WebGL renderer**: GPU-accelerated rendering for smoother animations

### Content Expansion
- [ ] **Regional catalogs**: European, North American, East Asian mushroom sets
- [ ] **Difficulty tiers for facts**: Beginner/intermediate/advanced educational content
- [ ] **Foraging safety module**: Critical look-alike warnings (Death Cap vs Paddy Straw, etc.)
- [ ] **Cooking tips**: Culinary preparation notes for edible species
- [ ] **Ecology deepdives**: Mycelial network visualizations, decomposition animations

---

## Architecture Notes

### Current Tech Stack
- **Language**: Rust → WebAssembly (wasm32-unknown-unknown)
- **Build**: Trunk 0.21.14
- **Rendering**: HTML5 Canvas 2D (960×540 desktop / 720×960 mobile)
- **State**: `AppState` struct with `GamePhase` enum state machine
- **API**: iNaturalist API v1 (observations endpoint, taxon_id=47170 for fungi)
- **Leaderboard**: Local SQLite via separate API server

### Key Design Decisions
1. **Canvas-only game UI** — no DOM elements in the game area (consistent rendering)
2. **iNat photo loading** — prefer taxon `default_photo` with CC license for CORS reliability
3. **Species attributes for unknown species** — cascading lookup: built-in catalog → extended DB → genus defaults → ultimate fallback
4. **Deterministic "randomness"** — mushroom selection uses sequence-based index, not Math.random()
5. **Variety axis includes iNaturalist** — treated as a 4th option alongside Small/Medium/Large
