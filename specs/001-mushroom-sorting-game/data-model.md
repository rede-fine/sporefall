# Data Model: Sporefall Mushroom Sorting Game

## Overview

Sporefall needs a curated canonical mushroom record that can support both the
scientific dataset pipeline and the simplified bucket logic used during play.

## Entities

### MushroomRecord

Represents one curated species entry used by the ingestion pipeline.

| Field | Type | Notes |
|------|------|-------|
| `record_id` | string | Stable internal identifier |
| `catalogue_of_life_id` | string | Primary taxonomy backbone id when available |
| `index_fungorum_id` | string | Optional fungal nomenclature cross-check |
| `scientific_name` | string | Canonical display name for taxonomy-aware play |
| `common_name` | string | Beginner-facing label when available |
| `region` | string | Curated regional scope marker |
| `ecological_guild` | enum | Saprotrophic, Mycorrhizal, Parasitic, Endophytic, or curated extension |
| `taxonomic_group` | string | Game-ready grouping derived from the chosen taxonomy bucket rules |
| `functional_group` | string | Culinary or functional grouping used by the game |
| `season_window` | list<string> | One or more player-facing seasonal labels |
| `color_group` | string | Obvious visual category used for beginner-friendly play |
| `image_asset_id` | string | Pointer to curated public image asset |
| `image_license` | string | License required for public redistribution |
| `image_attribution` | string | Attribution text stored with the asset |
| `source_provenance` | list<string> | Sources used to derive the final record |

### GameCard

Represents a playable prompt emitted to the browser game.

| Field | Type | Notes |
|------|------|-------|
| `card_id` | string | Stable game-facing id |
| `record_id` | string | Reference to canonical mushroom record |
| `clue_modes` | object | Available common name, scientific name, and image clues |
| `accepted_buckets` | map<string, string> | Bucket label per category system |
| `difficulty_tier` | string | Used for level rotation |

### BucketSet

Represents one active category mode shown in a session.

| Field | Type | Notes |
|------|------|-------|
| `bucket_set_id` | string | Stable identifier |
| `category_type` | enum | Ecology, Taxonomy, Function, Season, Color |
| `labels` | list<string> | Visible bucket labels |
| `required_clues` | list<string> | Minimum clues needed to keep the set solvable |

### LevelProfile

Represents one progression step.

| Field | Type | Notes |
|------|------|-------|
| `level_id` | string | Stable identifier |
| `active_bucket_sets` | list<string> | Bucket systems used by the level |
| `allowed_clue_modes` | list<string> | Clues available during the level |
| `spawn_pool` | list<string> | Game cards eligible for the level |

## Validation Rules

- Every `GameCard` must map cleanly to every bucket set in which it can appear.
- Every public image must carry explicit attribution and license data.
- Every playable session must contain at least one clue set that makes the
  chosen bucket configuration solvable.
- Canonical taxonomy identifiers remain in the dataset even when the player only
  sees beginner-friendly labels.
