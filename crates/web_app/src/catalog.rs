use game_core::FallingMushroom;

/// Available category systems for bucket selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryMode {
    Ecology,
    Color,
    Season,
    Function,
}

impl CategoryMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Ecology => "Ecological Role",
            Self::Color => "Cap Color",
            Self::Season => "Peak Season",
            Self::Function => "Culinary Type",
        }
    }

    pub fn labels(&self) -> &'static [&'static str] {
        match self {
            Self::Ecology => &["Mycorrhizal", "Saprotrophic", "Parasitic", "Endophytic"],
            Self::Color => &["Red/Orange", "Brown/Tan", "White/Cream", "Yellow/Gold"],
            Self::Season => &["Spring", "Summer", "Autumn", "Winter"],
            Self::Function => &["Choice Edible", "Medicinal", "Toxic", "Inedible"],
        }
    }

    pub fn all() -> &'static [CategoryMode] {
        &[Self::Ecology, Self::Color, Self::Season, Self::Function]
    }
}

/// A mushroom entry with data for all category systems.
pub struct CatalogEntry {
    pub id: &'static str,
    pub display_name: &'static str,
    pub image_key: &'static str,
    /// Target lane index per category mode: [ecology, color, season, function]
    pub targets: [usize; 4],
}

impl CatalogEntry {
    pub fn target_for(&self, mode: CategoryMode) -> usize {
        match mode {
            CategoryMode::Ecology => self.targets[0],
            CategoryMode::Color => self.targets[1],
            CategoryMode::Season => self.targets[2],
            CategoryMode::Function => self.targets[3],
        }
    }
}

/// MVP dataset: 12 mushrooms with classifications across all category systems.
pub const MUSHROOM_CATALOG: &[CatalogEntry] = &[
    CatalogEntry { id: "chanterelle", display_name: "Chanterelle", image_key: "chanterelle", targets: [0, 3, 1, 0] },
    CatalogEntry { id: "fly-agaric", display_name: "Fly Agaric", image_key: "fly-agaric", targets: [0, 0, 2, 2] },
    CatalogEntry { id: "king-bolete", display_name: "King Bolete", image_key: "king-bolete", targets: [0, 1, 1, 0] },
    CatalogEntry { id: "oyster", display_name: "Oyster Mushroom", image_key: "oyster", targets: [1, 2, 2, 0] },
    CatalogEntry { id: "shiitake", display_name: "Shiitake", image_key: "shiitake", targets: [1, 1, 2, 0] },
    CatalogEntry { id: "turkey-tail", display_name: "Turkey Tail", image_key: "turkey-tail", targets: [1, 1, 2, 1] },
    CatalogEntry { id: "honey-fungus", display_name: "Honey Fungus", image_key: "honey-fungus", targets: [2, 3, 2, 3] },
    CatalogEntry { id: "chaga", display_name: "Chaga", image_key: "chaga", targets: [2, 1, 3, 1] },
    CatalogEntry { id: "cordyceps", display_name: "Cordyceps", image_key: "cordyceps", targets: [2, 0, 1, 1] },
    CatalogEntry { id: "morel", display_name: "Morel", image_key: "morel", targets: [0, 1, 0, 0] },
    CatalogEntry { id: "death-cap", display_name: "Death Cap", image_key: "death-cap", targets: [3, 2, 2, 2] },
    CatalogEntry { id: "reishi", display_name: "Reishi", image_key: "reishi", targets: [1, 0, 1, 1] },
];

/// Pick a mushroom for the given category mode using varied selection.
pub fn pick_mushroom(sequence_number: usize, mode: CategoryMode) -> FallingMushroom {
    let index = (sequence_number * 7 + sequence_number / 3) % MUSHROOM_CATALOG.len();
    let entry = &MUSHROOM_CATALOG[index];
    FallingMushroom {
        id: entry.id.to_owned(),
        display_name: entry.display_name.to_owned(),
        target_lane: entry.target_for(mode),
        image_key: entry.image_key.to_owned(),
    }
}
