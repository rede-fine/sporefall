# Research: Sporefall Mushroom Sorting Game

## Decision Summary

- Use Rust compiled to WebAssembly for gameplay logic.
- Host the game as a static site on GitHub Pages.
- Scope v1 to a curated regional mushroom dataset rather than a global corpus.
- Build the dataset from multiple sources because no single source covers the
  needed taxonomy, ecology, seasonality, images, and licensing constraints.

## Hosting and Runtime

GitHub Pages can host Sporefall because the game fits a static-site model:
HTML, CSS, JavaScript glue, WebAssembly, JSON data, and images. A backend is not
required for the v1 single-player experience, but this means shared leaderboards
or server-side validation stay out of scope.

## Language and Architecture

Rust is acceptable here even though it adds build complexity because the goal is
to try Rust deliberately and the browser target is compatible through WASM. The
recommended architecture is a tested Rust game core plus a very thin browser/WASM
adapter layer.

## Mushroom Data Sources

### Taxonomy Backbone

Use Catalogue of Life as the primary taxonomy backbone because it is curated by
taxonomic experts, versioned, and released with DOI-backed snapshots.

### Fungal Name Cross-Check

Use Index Fungorum, and when needed Species Fungorum, to validate fungal names
and resolve nomenclature conflicts that arise during curation.

### Ecological Guilds

Use FUNGuild as the scientific source for ecological categories such as
saprotrophic, mycorrhizal, parasitic, and endophytic roles.

### Occurrence and Seasonality

Use MyCoPortal specimen and occurrence data to infer regional presence and
seasonality for the curated v1 species list. This is especially practical if v1
targets North America.

### Public Image Strategy

Use Wikimedia Commons as the default public image source because image reuse on a
public GitHub Pages deployment is easier to audit and attribute there than on
many community-observation platforms.

## Scope Decision

Choose a curated regional dataset for v1. This keeps the image set reviewable,
reduces taxonomy cleanup, keeps asset size manageable for GitHub Pages, and
raises confidence that the clue combinations remain playable.

## Risks to Carry Forward

- Ecological guild data may not exist cleanly for every candidate species.
- Seasonality inferred from occurrence data may need manual review.
- Public image licensing must be stored per record, not assumed globally.
- The dataset pipeline is a first-class dependency for gameplay quality.
