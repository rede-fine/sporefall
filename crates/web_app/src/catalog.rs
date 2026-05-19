use game_core::FallingMushroom;

/// Category labels displayed on each lane.
pub const CATEGORY_LABELS: &[&str] = &["Mycorrhizal", "Saprotrophic", "Parasitic", "Edible-Choice"];

/// A mushroom entry in the game catalog.
pub struct CatalogEntry {
    pub id: &'static str,
    pub display_name: &'static str,
    /// Index into CATEGORY_LABELS for the correct lane.
    pub target_lane: usize,
}

/// Hardcoded MVP dataset: 12 mushrooms across 4 ecological categories.
pub const MUSHROOM_CATALOG: &[CatalogEntry] = &[
    // Mycorrhizal (lane 0) - form symbiotic root partnerships
    CatalogEntry { id: "chanterelle", display_name: "Chanterelle", target_lane: 0 },
    CatalogEntry { id: "fly-agaric", display_name: "Fly Agaric", target_lane: 0 },
    CatalogEntry { id: "king-bolete", display_name: "King Bolete", target_lane: 0 },
    // Saprotrophic (lane 1) - decompose dead organic matter
    CatalogEntry { id: "oyster", display_name: "Oyster Mushroom", target_lane: 1 },
    CatalogEntry { id: "shiitake", display_name: "Shiitake", target_lane: 1 },
    CatalogEntry { id: "turkey-tail", display_name: "Turkey Tail", target_lane: 1 },
    // Parasitic (lane 2) - attack living hosts
    CatalogEntry { id: "honey-fungus", display_name: "Honey Fungus", target_lane: 2 },
    CatalogEntry { id: "chaga", display_name: "Chaga", target_lane: 2 },
    CatalogEntry { id: "cordyceps", display_name: "Cordyceps", target_lane: 2 },
    // Edible-Choice (lane 3) - prized culinary mushrooms
    CatalogEntry { id: "morel", display_name: "Morel", target_lane: 3 },
    CatalogEntry { id: "matsutake", display_name: "Matsutake", target_lane: 3 },
    CatalogEntry { id: "truffle", display_name: "Black Truffle", target_lane: 3 },
];

/// Pick a mushroom from the catalog using a simple rotation with offset.
pub fn pick_mushroom(sequence_number: usize) -> FallingMushroom {
    // Use a stride that doesn't align with lane_count to feel varied
    let index = (sequence_number * 7 + sequence_number / 4) % MUSHROOM_CATALOG.len();
    let entry = &MUSHROOM_CATALOG[index];
    FallingMushroom {
        id: entry.id.to_owned(),
        display_name: entry.display_name.to_owned(),
        target_lane: entry.target_lane,
    }
}
