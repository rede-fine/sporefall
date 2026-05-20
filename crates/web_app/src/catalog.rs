use game_core::FallingMushroom;
use std::collections::HashSet;

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

    /// Level ordering: Color (1) → Function (2) → Ecology (3) → Season (4)
    pub fn for_level(level: usize) -> Self {
        match level {
            0 => Self::Color,
            1 => Self::Function,
            2 => Self::Ecology,
            _ => Self::Season,
        }
    }
}

/// How many mushrooms are available per variety set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variety {
    Small,  // 12 mushrooms (core set)
    Medium, // 20 mushrooms
    Large,  // 28+ mushrooms
}

impl Variety {
    pub fn all() -> &'static [Variety] {
        &[Self::Small, Self::Medium, Self::Large]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Small => "12 species — learn the basics",
            Self::Medium => "20 species — more variety",
            Self::Large => "28 species — full challenge",
        }
    }

    pub fn count(&self) -> usize {
        match self {
            Self::Small => 12,
            Self::Medium => 20,
            Self::Large => 28,
        }
    }
}

/// A mushroom entry with data for all category systems.
pub struct CatalogEntry {
    pub id: &'static str,
    pub display_name: &'static str,
    pub latin_name: &'static str,
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

/// Full mushroom catalog: 28 species.
/// First 12 = Small set, first 20 = Medium set, all 28 = Large set.
pub const MUSHROOM_CATALOG: &[CatalogEntry] = &[
    // === SMALL SET (12) ===
    CatalogEntry { id: "chanterelle", display_name: "Chanterelle", latin_name: "Cantharellus cibarius", image_key: "chanterelle", targets: [0, 3, 1, 0] },
    CatalogEntry { id: "fly-agaric", display_name: "Fly Agaric", latin_name: "Amanita muscaria", image_key: "fly-agaric", targets: [0, 0, 2, 2] },
    CatalogEntry { id: "king-bolete", display_name: "King Bolete", latin_name: "Boletus edulis", image_key: "king-bolete", targets: [0, 1, 1, 0] },
    CatalogEntry { id: "oyster", display_name: "Oyster Mushroom", latin_name: "Pleurotus ostreatus", image_key: "oyster", targets: [1, 2, 2, 0] },
    CatalogEntry { id: "shiitake", display_name: "Shiitake", latin_name: "Lentinula edodes", image_key: "shiitake", targets: [1, 1, 2, 0] },
    CatalogEntry { id: "turkey-tail", display_name: "Turkey Tail", latin_name: "Trametes versicolor", image_key: "turkey-tail", targets: [1, 1, 2, 1] },
    CatalogEntry { id: "honey-fungus", display_name: "Honey Fungus", latin_name: "Armillaria mellea", image_key: "honey-fungus", targets: [2, 3, 2, 3] },
    CatalogEntry { id: "chaga", display_name: "Chaga", latin_name: "Inonotus obliquus", image_key: "chaga", targets: [2, 1, 3, 1] },
    CatalogEntry { id: "cordyceps", display_name: "Cordyceps", latin_name: "Ophiocordyceps sinensis", image_key: "cordyceps", targets: [2, 0, 1, 1] },
    CatalogEntry { id: "morel", display_name: "Morel", latin_name: "Morchella esculenta", image_key: "morel", targets: [0, 1, 0, 0] },
    CatalogEntry { id: "death-cap", display_name: "Death Cap", latin_name: "Amanita phalloides", image_key: "death-cap", targets: [3, 2, 2, 2] },
    CatalogEntry { id: "reishi", display_name: "Reishi", latin_name: "Ganoderma lucidum", image_key: "reishi", targets: [1, 0, 1, 1] },
    // === MEDIUM SET (+8 = 20) ===
    CatalogEntry { id: "enoki", display_name: "Enoki", latin_name: "Flammulina velutipes", image_key: "enoki", targets: [1, 2, 3, 0] },
    CatalogEntry { id: "lions-mane", display_name: "Lion's Mane", latin_name: "Hericium erinaceus", image_key: "lions-mane", targets: [1, 2, 2, 1] },
    CatalogEntry { id: "matsutake", display_name: "Matsutake", latin_name: "Tricholoma matsutake", image_key: "matsutake", targets: [0, 2, 2, 0] },
    CatalogEntry { id: "maitake", display_name: "Maitake", latin_name: "Grifola frondosa", image_key: "maitake", targets: [2, 1, 2, 1] },
    CatalogEntry { id: "destroying-angel", display_name: "Destroying Angel", latin_name: "Amanita virosa", image_key: "destroying-angel", targets: [0, 2, 1, 2] },
    CatalogEntry { id: "porcini", display_name: "Bay Bolete", latin_name: "Imleria badia", image_key: "porcini", targets: [0, 1, 2, 0] },
    CatalogEntry { id: "chicken-of-woods", display_name: "Chicken of the Woods", latin_name: "Laetiporus sulphureus", image_key: "chicken-of-woods", targets: [2, 3, 1, 0] },
    CatalogEntry { id: "shaggy-ink-cap", display_name: "Shaggy Ink Cap", latin_name: "Coprinus comatus", image_key: "shaggy-ink-cap", targets: [1, 2, 2, 0] },
    // === LARGE SET (+8 = 28) ===
    CatalogEntry { id: "penny-bun", display_name: "Penny Bun", latin_name: "Boletus edulis var.", image_key: "penny-bun", targets: [0, 1, 2, 0] },
    CatalogEntry { id: "giant-puffball", display_name: "Giant Puffball", latin_name: "Calvatia gigantea", image_key: "giant-puffball", targets: [1, 2, 2, 0] },
    CatalogEntry { id: "jelly-ear", display_name: "Jelly Ear", latin_name: "Auricularia auricula-judae", image_key: "jelly-ear", targets: [1, 1, 3, 3] },
    CatalogEntry { id: "birch-polypore", display_name: "Birch Polypore", latin_name: "Fomitopsis betulina", image_key: "birch-polypore", targets: [2, 2, 2, 1] },
    CatalogEntry { id: "false-morel", display_name: "False Morel", latin_name: "Gyromitra esculenta", image_key: "false-morel", targets: [0, 1, 0, 2] },
    CatalogEntry { id: "wood-ear", display_name: "Wood Ear", latin_name: "Auricularia polytricha", image_key: "wood-ear", targets: [1, 1, 2, 0] },
    CatalogEntry { id: "agarikon", display_name: "Agarikon", latin_name: "Laricifomes officinalis", image_key: "agarikon", targets: [2, 2, 1, 1] },
    CatalogEntry { id: "jack-o-lantern", display_name: "Jack O'Lantern", latin_name: "Omphalotus olearius", image_key: "jack-o-lantern", targets: [1, 0, 2, 2] },
];

/// Get mushrooms for the given variety level.
pub fn catalog_for_variety(variety: Variety) -> &'static [CatalogEntry] {
    &MUSHROOM_CATALOG[..variety.count()]
}

/// Pick a mushroom that hasn't been correctly sorted yet.
/// Returns None if all mushrooms in the set have been sorted.
pub fn pick_mushroom(
    sequence_number: usize,
    mode: CategoryMode,
    variety: Variety,
    sorted_ids: &HashSet<String>,
) -> Option<FallingMushroom> {
    let catalog = catalog_for_variety(variety);
    let available: Vec<&CatalogEntry> = catalog
        .iter()
        .filter(|e| !sorted_ids.contains(e.id))
        .collect();

    if available.is_empty() {
        return None;
    }

    let index = (sequence_number * 7 + sequence_number / 3) % available.len();
    let entry = available[index];
    Some(FallingMushroom {
        id: entry.id.to_owned(),
        display_name: entry.display_name.to_owned(),
        latin_name: entry.latin_name.to_owned(),
        target_lane: entry.target_for(mode),
        image_key: entry.image_key.to_owned(),
    })
}
