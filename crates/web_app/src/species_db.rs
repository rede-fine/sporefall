/// Extended species database for assigning game attributes to fungi
/// observed on iNaturalist that aren't in the curated catalog.
///
/// Attributes: [ecology, color, season, function]
/// Ecology: 0=Mycorrhizal, 1=Saprotrophic, 2=Parasitic, 3=Endophytic
/// Color: 0=Red/Orange, 1=Brown/Tan, 2=White/Cream, 3=Yellow/Gold
/// Season: 0=Spring, 1=Summer, 2=Autumn, 3=Winter
/// Function: 0=Choice Edible, 1=Medicinal, 2=Toxic, 3=Inedible

/// Returns targets [ecology, color, season, function] for a given latin name.
/// Falls back to heuristic if not in the extended database.
pub fn lookup_targets(latin_name: &str) -> [usize; 4] {
    let key = latin_name.to_lowercase();

    // Check extended database first
    for &(name, targets) in EXTENDED_DB {
        if key.starts_with(&name.to_lowercase()) {
            return targets;
        }
    }

    // Genus-level fallbacks
    let genus = key.split_whitespace().next().unwrap_or("");
    for &(g, targets) in GENUS_DEFAULTS {
        if genus == g {
            return targets;
        }
    }

    // Ultimate fallback: Saprotrophic, Brown/Tan, Autumn, Inedible
    [1, 1, 2, 3]
}

/// Extended database of fungi with known game attributes.
/// Format: (latin_name_prefix, [ecology, color, season, function])
/// Sources: MycoBank, Wikipedia, field guides (Roger Phillips, David Arora)
const EXTENDED_DB: &[(&str, [usize; 4])] = &[
    // === Already in main catalog (28 species) ===
    ("Cantharellus cibarius", [0, 3, 1, 0]),   // Chanterelle
    ("Amanita muscaria", [0, 0, 2, 2]),         // Fly Agaric
    ("Boletus edulis", [0, 1, 1, 0]),           // King Bolete / Penny Bun
    ("Pleurotus ostreatus", [1, 2, 2, 0]),      // Oyster Mushroom
    ("Lentinula edodes", [1, 1, 2, 0]),         // Shiitake
    ("Trametes versicolor", [1, 1, 2, 1]),      // Turkey Tail
    ("Armillaria mellea", [2, 3, 2, 3]),        // Honey Fungus
    ("Inonotus obliquus", [2, 1, 3, 1]),        // Chaga
    ("Ophiocordyceps sinensis", [2, 0, 1, 1]),  // Cordyceps
    ("Morchella esculenta", [0, 1, 0, 0]),      // Morel
    ("Amanita phalloides", [0, 2, 2, 2]),       // Death Cap - mycorrhizal with oaks/beeches
    ("Ganoderma lucidum", [1, 0, 1, 1]),        // Reishi
    ("Flammulina velutipes", [1, 2, 3, 0]),     // Enoki
    ("Hericium erinaceus", [1, 2, 2, 1]),       // Lion's Mane
    ("Tricholoma matsutake", [0, 2, 2, 0]),     // Matsutake
    ("Grifola frondosa", [2, 1, 2, 1]),         // Maitake
    ("Amanita virosa", [0, 2, 1, 2]),           // Destroying Angel
    ("Imleria badia", [0, 1, 2, 0]),            // Bay Bolete
    ("Laetiporus sulphureus", [2, 3, 1, 0]),    // Chicken of the Woods
    ("Coprinus comatus", [1, 2, 2, 0]),         // Shaggy Ink Cap
    ("Calvatia gigantea", [1, 2, 2, 0]),        // Giant Puffball
    ("Auricularia auricula-judae", [1, 1, 3, 0]), // Jelly Ear - edible (common in Asian cuisine)
    ("Fomitopsis betulina", [2, 2, 2, 1]),      // Birch Polypore
    ("Gyromitra esculenta", [0, 1, 0, 2]),      // False Morel
    ("Auricularia polytricha", [1, 1, 2, 0]),   // Wood Ear
    ("Laricifomes officinalis", [2, 2, 1, 1]),  // Agarikon
    ("Omphalotus olearius", [1, 0, 2, 2]),      // Jack O'Lantern

    // === Extended species (common European & worldwide fungi) ===
    // Boletes
    ("Suillus luteus", [0, 1, 2, 0]),           // Slippery Jack - mycorrhizal, brown, autumn, edible
    ("Suillus grevillei", [0, 3, 2, 0]),        // Larch Bolete - mycorrhizal, yellow, autumn, edible
    ("Suillus bovinus", [0, 1, 2, 0]),          // Jersey Cow Bolete
    ("Leccinum scabrum", [0, 1, 2, 0]),         // Brown Birch Bolete
    ("Leccinum versipelle", [0, 0, 2, 0]),      // Orange Birch Bolete
    ("Leccinum aurantiacum", [0, 0, 2, 0]),     // Red-capped Scaber Stalk
    ("Xerocomellus chrysenteron", [0, 1, 2, 0]), // Red-cracking Bolete
    ("Xerocomus subtomentosus", [0, 3, 2, 0]),  // Suede Bolete
    ("Tylopilus felleus", [0, 1, 2, 3]),        // Bitter Bolete - inedible (too bitter)
    ("Chalciporus piperatus", [0, 1, 2, 3]),    // Peppery Bolete

    // Amanitas
    ("Amanita rubescens", [0, 0, 2, 0]),        // Blusher - edible when cooked
    ("Amanita caesarea", [0, 0, 1, 0]),         // Caesar's Mushroom
    ("Amanita pantherina", [0, 1, 2, 2]),       // Panther Cap - toxic
    ("Amanita citrina", [0, 3, 2, 3]),          // False Death Cap - inedible
    ("Amanita fulva", [0, 1, 1, 0]),            // Tawny Grisette
    ("Amanita vaginata", [0, 2, 2, 0]),         // Grisette

    // Agaricus (field mushrooms)
    ("Agaricus bitorquis", [1, 2, 1, 0]),       // Pavement Mushroom - saprotrophic, white, summer, edible
    ("Agaricus campestris", [1, 2, 2, 0]),      // Field Mushroom
    ("Agaricus arvensis", [1, 2, 2, 0]),        // Horse Mushroom
    ("Agaricus augustus", [1, 1, 2, 0]),         // The Prince
    ("Agaricus silvicola", [1, 2, 2, 0]),       // Wood Mushroom
    ("Agaricus xanthodermus", [1, 2, 2, 2]),    // Yellow-staining Mushroom - toxic

    // Russulas & Lactarius
    ("Lactarius deliciosus", [0, 0, 2, 0]),     // Saffron Milkcap - mycorrhizal, orange, autumn, edible
    ("Lactarius deterrimus", [0, 0, 2, 0]),     // False Saffron Milkcap
    ("Lactarius torminosus", [0, 0, 2, 2]),     // Woolly Milkcap - toxic raw
    ("Lactarius rufus", [0, 0, 2, 3]),          // Rufous Milkcap - inedible
    ("Russula emetica", [0, 0, 2, 2]),          // The Sickener
    ("Russula cyanoxantha", [0, 1, 2, 0]),      // Charcoal Burner
    ("Russula vesca", [0, 1, 1, 0]),            // Bare-toothed Russula
    ("Russula virescens", [0, 2, 1, 0]),        // Green Cracking Russula
    ("Russula nigricans", [0, 1, 2, 3]),        // Blackening Russula

    // Cortinarius
    ("Cortinarius violaceus", [0, 1, 2, 3]),    // Violet Webcap
    ("Cortinarius rubellus", [0, 0, 2, 2]),     // Deadly Webcap

    // Coprinoids
    ("Coprinellus micaceus", [1, 1, 2, 3]),     // Glistening Inkcap
    ("Coprinopsis atramentaria", [1, 1, 2, 3]), // Common Inkcap

    // Tricholoma
    ("Tricholoma equestre", [0, 3, 2, 2]),      // Yellow Knight - potentially toxic
    ("Tricholoma terreum", [0, 1, 2, 0]),       // Grey Knight

    // Macrolepiota
    ("Macrolepiota procera", [1, 1, 2, 0]),     // Parasol Mushroom
    ("Chlorophyllum rhacodes", [1, 1, 2, 0]),   // Shaggy Parasol

    // Pholiota & relatives
    ("Pholiota squarrosa", [2, 3, 2, 3]),       // Shaggy Scalycap
    ("Kuehneromyces mutabilis", [1, 1, 2, 0]),  // Sheathed Woodtuft
    ("Hypholoma fasciculare", [1, 3, 2, 2]),    // Sulphur Tuft - toxic

    // Clitocybe & relatives
    ("Pseudoclitocybe cyathiformis", [1, 1, 2, 3]), // The Goblet - saprotrophic, brown, autumn, inedible
    ("Clitocybe nebularis", [1, 2, 2, 3]),      // Clouded Agaric
    ("Clitocybe rivulosa", [1, 2, 2, 2]),       // Fool's Funnel - toxic

    // Polypores & bracket fungi
    ("Fomes fomentarius", [2, 1, 2, 1]),        // Tinder Fungus - medicinal
    ("Ganoderma applanatum", [2, 1, 2, 1]),     // Artist's Bracket
    ("Trametes hirsuta", [1, 2, 2, 3]),         // Hairy Bracket
    ("Stereum hirsutum", [1, 3, 2, 3]),         // Hairy Curtain Crust
    ("Bjerkandera adusta", [1, 1, 2, 3]),       // Smoky Bracket
    ("Daedaleopsis confragosa", [1, 1, 2, 3]),  // Blushing Bracket

    // Stinkhorns & earth stars
    ("Phallus impudicus", [1, 2, 1, 3]),        // Common Stinkhorn
    ("Geastrum triplex", [1, 1, 2, 3]),         // Collared Earthstar
    ("Clathrus archeri", [1, 0, 2, 3]),         // Devil's Fingers

    // Chanterelle relatives
    ("Cantharellus tubaeformis", [0, 1, 2, 0]), // Trumpet Chanterelle
    ("Craterellus cornucopioides", [0, 1, 2, 0]), // Horn of Plenty
    ("Hydnum repandum", [0, 2, 2, 0]),          // Hedgehog Mushroom

    // Puffballs & clubs
    ("Lycoperdon perlatum", [1, 2, 2, 0]),      // Common Puffball
    ("Lycoperdon pyriforme", [1, 1, 2, 0]),     // Stump Puffball
    ("Scleroderma citrinum", [0, 3, 2, 2]),     // Common Earthball - toxic

    // Waxcaps
    ("Hygrocybe coccinea", [1, 0, 2, 3]),       // Scarlet Waxcap
    ("Hygrocybe psittacina", [1, 0, 2, 3]),     // Parrot Waxcap
    ("Cuphophyllus virgineus", [1, 2, 2, 3]),   // Snowy Waxcap

    // Coral & jelly fungi
    ("Ramaria botrytis", [0, 2, 2, 0]),         // Cauliflower Coral
    ("Clavulina cristata", [0, 2, 2, 3]),       // Crested Coral
    ("Tremella mesenterica", [2, 3, 3, 3]),     // Yellow Brain Fungus
    ("Exidia glandulosa", [1, 1, 3, 3]),        // Black Witch's Butter
    ("Calocera viscosa", [1, 3, 2, 3]),         // Yellow Stagshorn

    // Morels & cups
    ("Morchella elata", [0, 1, 0, 0]),          // Black Morel
    ("Helvella crispa", [0, 2, 2, 3]),          // White Saddle
    ("Helvella lacunosa", [0, 1, 2, 3]),        // Elfin Saddle
    ("Peziza vesiculosa", [1, 1, 0, 3]),        // Bladder Cup
    ("Sarcoscypha coccinea", [1, 0, 0, 3]),     // Scarlet Elf Cup

    // Miscellaneous well-known species
    ("Lepista nuda", [1, 1, 2, 0]),             // Wood Blewit
    ("Clitopilus prunulus", [1, 2, 2, 0]),      // The Sweetbread Mushroom
    ("Mycena haematopus", [1, 0, 2, 3]),        // Bleeding Mycena
    ("Mycena pura", [1, 2, 2, 2]),              // Lilac Bonnet - mildly toxic
    ("Panellus stipticus", [1, 1, 2, 3]),       // Bitter Oyster
    ("Pluteus cervinus", [1, 1, 2, 0]),         // Deer Shield
    ("Pleurotus pulmonarius", [1, 2, 1, 0]),    // Lung Oyster
    ("Agrocybe aegerita", [1, 1, 2, 0]),        // Pioppino
    ("Stropharia aeruginosa", [1, 0, 2, 3]),    // Verdigris Agaric
    ("Psilocybe semilanceata", [1, 1, 2, 2]),   // Liberty Cap - psychoactive=toxic for game
    ("Schizophyllum commune", [1, 2, 2, 3]),    // Split Gill
    ("Daldinia concentrica", [1, 1, 2, 3]),     // King Alfred's Cakes
    ("Xylaria hypoxylon", [1, 1, 3, 3]),        // Candlesnuff Fungus
    ("Xylaria polymorpha", [1, 1, 2, 3]),       // Dead Man's Fingers
    ("Nectria cinnabarina", [2, 0, 2, 3]),      // Coral Spot
    ("Fistulina hepatica", [2, 0, 2, 0]),       // Beefsteak Fungus
    ("Sparassis crispa", [2, 2, 2, 0]),         // Cauliflower Fungus
    ("Meripilus giganteus", [2, 1, 2, 3]),       // Giant Polypore
    ("Polyporus squamosus", [2, 1, 0, 0]),      // Dryad's Saddle
    ("Cerioporus squamosus", [2, 1, 0, 0]),     // Dryad's Saddle (new name)

    // Common European/German species (for erlizzard)
    ("Infundibulicybe geotropa", [1, 2, 2, 0]), // Trooping Funnel
    ("Paralepista flaccida", [1, 0, 2, 3]),     // Tawny Funnel
    ("Clitocybe gibba", [1, 1, 2, 0]),          // Common Funnel
    ("Gymnopus dryophilus", [1, 1, 1, 3]),      // Russet Toughshank
    ("Marasmius oreades", [1, 1, 1, 0]),        // Fairy Ring Champignon
    ("Tubaria furfuracea", [1, 1, 3, 3]),       // Scurfy Twiglet
    ("Psathyrella candolleana", [1, 1, 1, 3]),  // Pale Brittlestem
    ("Conocybe tenera", [1, 1, 1, 3]),          // Common Conecap
    ("Bolbitius titubans", [1, 3, 1, 3]),       // Yellow Fieldcap
    ("Hebeloma crustuliniforme", [0, 1, 2, 2]), // Poison Pie
    ("Inocybe geophylla", [0, 2, 2, 2]),        // White Fibrecap - toxic
    ("Entoloma sinuatum", [1, 2, 2, 2]),        // Livid Pinkgill - toxic

    // === North American species ===
    ("Amanita jacksonii", [0, 0, 1, 0]),        // American Caesar's Mushroom
    ("Amanita velosa", [0, 2, 0, 0]),           // Springtime Amanita
    ("Cantharellus formosus", [0, 3, 2, 0]),    // Pacific Golden Chanterelle
    ("Cantharellus lateritius", [0, 3, 1, 0]),  // Smooth Chanterelle
    ("Cantharellus cinnabarinus", [0, 0, 1, 0]),// Cinnabar Chanterelle
    ("Morchella americana", [0, 1, 0, 0]),      // Yellow Morel (American)
    ("Morchella punctipes", [0, 1, 0, 0]),      // Half-free Morel
    ("Morchella angusticeps", [0, 1, 0, 0]),    // Black Morel (American)
    ("Craterellus fallax", [0, 1, 2, 0]),       // Black Trumpet (American)
    ("Hydnum umbilicatum", [0, 0, 2, 0]),       // Sweet Tooth
    ("Boletus rex-veris", [0, 1, 0, 0]),        // Spring King Bolete
    ("Boletus fibrillosus", [0, 1, 2, 0]),      // Fibrillose Bolete
    ("Suillus americanus", [0, 3, 1, 0]),       // Chicken Fat Mushroom
    ("Leccinum insigne", [0, 0, 1, 0]),         // Aspen Bolete
    ("Lactarius indigo", [0, 0, 1, 0]),         // Indigo Milkcap (blue→treat as Red/Orange)
    ("Lactarius thyinos", [0, 0, 2, 0]),        // Orange Milkcap
    ("Russula xerampelina", [0, 0, 2, 0]),      // Shrimp Russula
    ("Russula brevipes", [0, 2, 2, 3]),         // Short-stemmed Russula
    ("Agaricus crocodilinus", [1, 2, 1, 0]),    // Crocodile Agaricus
    ("Chlorophyllum molybdites", [1, 2, 1, 2]), // Green-spored Parasol - toxic!
    ("Omphalotus illudens", [1, 0, 2, 2]),      // Eastern Jack O'Lantern
    ("Gymnopilus junonius", [1, 3, 2, 2]),      // Spectacular Rustgill
    ("Gymnopilus luteofolius", [1, 3, 2, 2]),   // Yellow-gilled Gymnopilus
    ("Trametes betulina", [1, 2, 2, 3]),        // Gilled Polypore
    ("Phaeolus schweinitzii", [2, 1, 2, 3]),    // Dyer's Polypore
    ("Bondarzewia berkeleyi", [2, 2, 2, 3]),    // Berkeley's Polypore
    ("Grifola frondosa", [2, 1, 2, 0]),         // Hen of the Woods (reclassify as edible)
    ("Ischnoderma resinosum", [1, 1, 2, 3]),    // Resinous Polypore
    ("Hericium americanum", [1, 2, 2, 1]),      // Bear's Head Tooth
    ("Hericium coralloides", [1, 2, 2, 1]),     // Coral Tooth
    ("Laetiporus cincinnatus", [2, 3, 1, 0]),   // White-pored Chicken of Woods

    // === East Asian species ===
    ("Ganoderma lingzhi", [1, 0, 1, 1]),        // Chinese Lingzhi
    ("Ganoderma sinense", [1, 1, 1, 1]),        // Black Lingzhi
    ("Cordyceps militaris", [2, 0, 2, 1]),      // Orange Caterpillar Fungus
    ("Wolfiporia extensa", [2, 2, 2, 1]),       // Poria/Fu Ling
    ("Tremella fuciformis", [2, 2, 2, 1]),      // Snow Fungus
    ("Dictyophora indusiata", [1, 2, 1, 0]),    // Bamboo Fungus
    ("Pholiota nameko", [1, 1, 2, 0]),          // Nameko
    ("Hypsizygus tessulatus", [1, 2, 2, 0]),    // Beech Mushroom (Shimeji)
    ("Hypsizygus marmoreus", [1, 2, 2, 0]),     // White Beech Mushroom
    ("Agrocybe cylindracea", [1, 1, 1, 0]),     // Poplar Mushroom
    ("Volvariella volvacea", [1, 2, 1, 0]),     // Paddy Straw Mushroom
    ("Pleurotus eryngii", [1, 2, 2, 0]),        // King Oyster
    ("Pleurotus citrinopileatus", [1, 3, 1, 0]),// Golden Oyster
    ("Pleurotus djamor", [1, 0, 1, 0]),         // Pink Oyster
    ("Cyclocybe aegerita", [1, 1, 2, 0]),       // Black Poplar Mushroom
    ("Stropharia rugosoannulata", [1, 0, 1, 0]),// Wine Cap (King Stropharia)

    // === Australian & Southern Hemisphere ===
    ("Omphalotus nidiformis", [1, 2, 2, 2]),    // Ghost Fungus (bioluminescent, toxic)
    ("Cortinarius archeri", [0, 0, 2, 2]),      // Australian Webcap
    ("Amanita xanthocephala", [0, 3, 2, 2]),    // Yellow Amanita
    ("Mycena chlorophos", [1, 0, 1, 3]),        // Bioluminescent Mycena
    ("Aseroe rubra", [1, 0, 1, 3]),             // Starfish Stinkhorn

    // === Tropical & subtropical ===
    ("Termitomyces titanicus", [0, 2, 1, 0]),   // Termite Mushroom (largest edible)
    ("Lentinus tigrinus", [1, 2, 1, 0]),        // Tiger Sawgill
    ("Schizophyllum commune", [1, 2, 2, 3]),    // Split Gill (cosmopolitan)
    ("Cookeina sulcipes", [1, 0, 1, 3]),        // Tropical Cup Fungus
    ("Favolaschia calocera", [1, 0, 2, 3]),     // Orange Pore Fungus

    // === Additional medicinal/functional fungi ===
    ("Inonotus hispidus", [2, 1, 1, 1]),        // Shaggy Bracket - medicinal
    ("Fomitopsis pinicola", [2, 0, 2, 1]),      // Red-belted Conk
    ("Phellinus igniarius", [2, 1, 2, 1]),      // Willow Bracket
    ("Piptoporus betulinus", [2, 2, 2, 1]),     // Birch Polypore (old name)
    ("Antrodia camphorata", [2, 0, 2, 1]),      // Camphor Fungus (Taiwan endemic)
    ("Sanghuangporus sanghuang", [2, 1, 2, 1]), // Sanghuang
    ("Trametes pubescens", [1, 2, 2, 3]),       // Velvet Bracket
    ("Lenzites betulina", [1, 1, 2, 3]),        // Birch Mazegill

    // === More toxic species (important for safety education) ===
    ("Galerina marginata", [1, 1, 2, 2]),       // Funeral Bell - deadly!
    ("Lepiota brunneoincarnata", [1, 1, 2, 2]), // Deadly Dapperling
    ("Lepiota subincarnata", [1, 1, 2, 2]),     // Fatal Dapperling
    ("Podostroma cornu-damae", [1, 0, 1, 2]),   // Poison Fire Coral - deadly!
    ("Trogia venenata", [1, 2, 1, 2]),          // Yunnan sudden death mushroom
    ("Paxillus involutus", [0, 1, 2, 2]),       // Brown Roll-rim - toxic
    ("Tricholoma pardinum", [0, 2, 2, 2]),      // Spotted Tricholoma - toxic
    ("Rubroboletus satanas", [0, 0, 1, 2]),     // Satan's Bolete - toxic

    // === More cultivated/commercial species ===
    ("Agaricus bisporus", [1, 2, 2, 0]),        // Button Mushroom/Portobello
    ("Agaricus subrufescens", [1, 1, 1, 1]),    // Almond Mushroom (ABM) - medicinal
    ("Pleurotus cornucopiae", [1, 2, 1, 0]),    // Branched Oyster
    ("Flammulina filiformis", [1, 2, 3, 0]),    // Cultivated Enoki (white form)
    ("Grifola frondosa", [2, 1, 2, 1]),         // Maitake (duplicate for lookup)
    ("Lentinula boryana", [1, 1, 2, 0]),        // Tropical Shiitake
    ("Auricularia heimuer", [1, 1, 2, 0]),      // Mu Er (Black Wood Ear)
    ("Ustilago maydis", [2, 1, 1, 0]),          // Huitlacoche (corn smut - delicacy)
];

/// Genus-level fallback targets for species not in the extended database.
const GENUS_DEFAULTS: &[(&str, [usize; 4])] = &[
    ("amanita", [0, 2, 2, 2]),     // Mycorrhizal, White, Autumn, Toxic (safer default)
    ("boletus", [0, 1, 2, 0]),     // Mycorrhizal, Brown, Autumn, Edible
    ("suillus", [0, 1, 2, 0]),     // Mycorrhizal, Brown, Autumn, Edible
    ("leccinum", [0, 1, 2, 0]),    // Mycorrhizal, Brown, Autumn, Edible
    ("russula", [0, 0, 2, 3]),     // Mycorrhizal, Red, Autumn, Inedible (safe default)
    ("lactarius", [0, 0, 2, 3]),   // Mycorrhizal, Red, Autumn, Inedible
    ("cortinarius", [0, 1, 2, 2]), // Mycorrhizal, Brown, Autumn, Toxic (many toxic)
    ("agaricus", [1, 2, 2, 0]),    // Saprotrophic, White, Autumn, Edible
    ("pleurotus", [1, 2, 2, 0]),   // Saprotrophic, White, Autumn, Edible
    ("mycena", [1, 1, 2, 3]),      // Saprotrophic, Brown, Autumn, Inedible
    ("coprinus", [1, 1, 2, 3]),    // Saprotrophic, Brown, Autumn, Inedible
    ("coprinellus", [1, 1, 2, 3]), // Saprotrophic, Brown, Autumn, Inedible
    ("coprinopsis", [1, 1, 2, 3]), // Saprotrophic, Brown, Autumn, Inedible
    ("clitocybe", [1, 2, 2, 3]),   // Saprotrophic, White, Autumn, Inedible
    ("hygrocybe", [1, 0, 2, 3]),   // Saprotrophic, Red, Autumn, Inedible
    ("tricholoma", [0, 1, 2, 3]),  // Mycorrhizal, Brown, Autumn, Inedible
    ("macrolepiota", [1, 1, 2, 0]),// Saprotrophic, Brown, Autumn, Edible
    ("trametes", [1, 1, 2, 1]),    // Saprotrophic, Brown, Autumn, Medicinal
    ("ganoderma", [2, 1, 2, 1]),   // Parasitic, Brown, Autumn, Medicinal
    ("fomes", [2, 1, 2, 1]),       // Parasitic, Brown, Autumn, Medicinal
    ("pholiota", [2, 3, 2, 3]),    // Parasitic, Yellow, Autumn, Inedible
    ("armillaria", [2, 3, 2, 3]),  // Parasitic, Yellow, Autumn, Inedible
    ("cantharellus", [0, 3, 1, 0]),// Mycorrhizal, Yellow, Summer, Edible
    ("craterellus", [0, 1, 2, 0]), // Mycorrhizal, Brown, Autumn, Edible
    ("hydnum", [0, 2, 2, 0]),      // Mycorrhizal, White, Autumn, Edible
    ("morchella", [0, 1, 0, 0]),   // Mycorrhizal, Brown, Spring, Edible
    ("hericium", [1, 2, 2, 1]),    // Saprotrophic, White, Autumn, Medicinal
    ("lycoperdon", [1, 2, 2, 0]),  // Saprotrophic, White, Autumn, Edible
    ("ramaria", [0, 2, 2, 3]),     // Mycorrhizal, White, Autumn, Inedible
    ("xylaria", [1, 1, 2, 3]),     // Saprotrophic, Brown, Autumn, Inedible
    ("lepista", [1, 1, 2, 0]),     // Saprotrophic, Brown, Autumn, Edible
    ("stropharia", [1, 0, 2, 3]),  // Saprotrophic, Red, Autumn, Inedible
    ("psilocybe", [1, 1, 2, 2]),   // Saprotrophic, Brown, Autumn, Toxic
    ("inocybe", [0, 1, 2, 2]),     // Mycorrhizal, Brown, Autumn, Toxic
    ("hebeloma", [0, 1, 2, 2]),    // Mycorrhizal, Brown, Autumn, Toxic
    ("entoloma", [1, 2, 2, 2]),    // Saprotrophic, White, Autumn, Toxic
    ("phallus", [1, 2, 1, 3]),     // Saprotrophic, White, Summer, Inedible
    ("fistulina", [2, 0, 2, 0]),   // Parasitic, Red, Autumn, Edible
    ("laetiporus", [2, 3, 1, 0]),  // Parasitic, Yellow, Summer, Edible
    ("polyporus", [2, 1, 0, 3]),   // Parasitic, Brown, Spring, Inedible
    ("cerioporus", [2, 1, 0, 0]),  // Parasitic, Brown, Spring, Edible
    ("sparassis", [2, 2, 2, 0]),   // Parasitic, White, Autumn, Edible
    ("calvatia", [1, 2, 2, 0]),    // Saprotrophic, White, Autumn, Edible
    ("scleroderma", [0, 3, 2, 2]), // Mycorrhizal, Yellow, Autumn, Toxic
    ("gyromitra", [0, 1, 0, 2]),   // Mycorrhizal, Brown, Spring, Toxic
    ("helvella", [0, 1, 2, 3]),    // Mycorrhizal, Brown, Autumn, Inedible
    ("tremella", [2, 3, 3, 3]),    // Parasitic, Yellow, Winter, Inedible
    ("auricularia", [1, 1, 2, 0]), // Saprotrophic, Brown, Autumn, Edible
    ("xerocomellus", [0, 1, 2, 0]),// Mycorrhizal, Brown, Autumn, Edible
    ("pseudoclitocybe", [1, 1, 2, 3]), // Saprotrophic, Brown, Autumn, Inedible
    ("gymnopilus", [1, 3, 2, 2]),  // Saprotrophic, Yellow, Autumn, Toxic
    ("galerina", [1, 1, 2, 2]),    // Saprotrophic, Brown, Autumn, Toxic (deadly!)
    ("lepiota", [1, 2, 2, 2]),     // Saprotrophic, White, Autumn, Toxic (many deadly)
    ("chlorophyllum", [1, 2, 2, 0]),// Saprotrophic, White, Autumn, Edible (except molybdites)
    ("volvariella", [1, 2, 1, 0]), // Saprotrophic, White, Summer, Edible
    ("termitomyces", [0, 2, 1, 0]),// Mycorrhizal, White, Summer, Edible
    ("phellinus", [2, 1, 2, 1]),   // Parasitic, Brown, Autumn, Medicinal
    ("fomitopsis", [2, 1, 2, 1]), // Parasitic, Brown, Autumn, Medicinal
    ("wolfiporia", [2, 2, 2, 1]), // Parasitic, White, Autumn, Medicinal
    ("sanghuangporus", [2, 1, 2, 1]), // Parasitic, Brown, Autumn, Medicinal
    ("lentinus", [1, 2, 1, 0]),    // Saprotrophic, White, Summer, Edible
    ("hypsizygus", [1, 2, 2, 0]), // Saprotrophic, White, Autumn, Edible
    ("cyclocybe", [1, 1, 2, 0]),  // Saprotrophic, Brown, Autumn, Edible
    ("agrocybe", [1, 1, 2, 0]),   // Saprotrophic, Brown, Autumn, Edible
    ("pholiota", [2, 3, 2, 3]),    // Parasitic, Yellow, Autumn, Inedible
    ("dictyophora", [1, 2, 1, 0]),// Saprotrophic, White, Summer, Edible
    ("bondarzewia", [2, 2, 2, 3]),// Parasitic, White, Autumn, Inedible
    ("omphalotus", [1, 0, 2, 2]), // Saprotrophic, Red/Orange, Autumn, Toxic
    ("meripilus", [2, 1, 2, 3]),  // Parasitic, Brown, Autumn, Inedible
    ("ustilago", [2, 1, 1, 0]),   // Parasitic, Brown, Summer, Edible (corn smut)
];
