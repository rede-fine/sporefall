use game_core::FallingMushroom;

/// Generate a fun/educational fact about a basket of mushrooms.
pub fn basket_fact(mushrooms: &[FallingMushroom]) -> String {
    let ids: Vec<&str> = mushrooms.iter().map(|m| m.id.as_str()).collect();

    // Death cap special rule
    if ids.contains(&"death-cap") {
        return "☠️ DANGER! A Death Cap (Amanita phalloides) snuck into your basket! \
                Just 30g can kill an adult. You'd have to throw EVERYTHING out — \
                no amount of cooking destroys its toxins!".to_owned();
    }

    // Fly agaric combo
    if ids.contains(&"fly-agaric") {
        return "🍄 Careful! Fly Agaric (Amanita muscaria) in your basket! \
                Vikings reportedly used it for 'berserker' rage. \
                It's toxic raw but some cultures detoxify it through parboiling.".to_owned();
    }

    // All edible combo
    if ids.iter().all(|id| matches!(*id, "chanterelle" | "king-bolete" | "oyster" | "shiitake" | "morel")) {
        return "👨‍🍳 A chef's dream basket! All choice edibles. \
                This combination would make an incredible wild mushroom risotto!".to_owned();
    }

    // Medicinal combo
    if ids.iter().any(|id| matches!(*id, "reishi" | "chaga" | "turkey-tail" | "cordyceps")) {
        let medicinals: Vec<&str> = ids.iter()
            .filter(|id| matches!(**id, "reishi" | "chaga" | "turkey-tail" | "cordyceps"))
            .copied()
            .collect();
        if medicinals.len() >= 2 {
            return "🧪 A medicinal powerhouse! Traditional Chinese Medicine has used \
                    these fungi for over 2000 years. Modern research is now confirming \
                    their immune-boosting properties.".to_owned();
        }
    }

    // Chanterelle + morel
    if ids.contains(&"chanterelle") && ids.contains(&"morel") {
        return "🌟 Chanterelles and Morels in one basket! These are the two most \
                prized wild mushrooms in European cuisine. Together they can sell \
                for over $100/kg at farmers markets!".to_owned();
    }

    // Cordyceps fact
    if ids.contains(&"cordyceps") {
        return "🐛 Cordyceps is a parasitic fungus that takes over insect bodies! \
                The 'zombie fungus' inspired the TV show The Last of Us. \
                Despite its horror origins, it's used as an energy supplement.".to_owned();
    }

    // Chaga fact
    if ids.contains(&"chaga") {
        return "🪨 Chaga looks like burnt charcoal on birch trees but is actually \
                one of the most antioxidant-rich substances on Earth — \
                scoring higher than acai berries on the ORAC scale!".to_owned();
    }

    // Turkey tail
    if ids.contains(&"turkey-tail") {
        return "🦃 Turkey Tail (Trametes versicolor) is the most researched \
                medicinal mushroom. Its extract PSK is an approved \
                anti-cancer adjuvant therapy in Japan!".to_owned();
    }

    // Honey fungus
    if ids.contains(&"honey-fungus") {
        return "🍯 The largest living organism on Earth is a Honey Fungus \
                (Armillaria ostoyae) in Oregon — it spans 2,385 acres and is \
                estimated to be 2,400 years old!".to_owned();
    }

    // King bolete
    if ids.contains(&"king-bolete") {
        return "👑 King Bolete (Boletus edulis) is called 'Porcini' in Italy, \
                'Cep' in France, and 'Steinpilz' in Germany. \
                It cannot be cultivated — only found wild!".to_owned();
    }

    // Generic facts
    let generic = [
        "🍄 Fungi are more closely related to animals than to plants! \
         They share a common ancestor from about 1 billion years ago.",
        "🌍 There are an estimated 3.8 million fungal species on Earth, \
         but only about 148,000 have been described by science.",
        "🌲 90% of all land plants depend on mycorrhizal fungi \
         for nutrient exchange — it's nature's original internet!",
        "💡 Some mushrooms are bioluminescent! Over 80 species glow \
         in the dark, possibly to attract insects that spread spores.",
    ];

    let index = mushrooms.iter().map(|m| m.id.len()).sum::<usize>() % generic.len();
    generic[index].to_owned()
}
