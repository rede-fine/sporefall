use std::collections::HashMap;
use web_sys::HtmlImageElement;

/// Preloads and caches mushroom images by image_key.
pub struct ImageCache {
    images: HashMap<String, HtmlImageElement>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }

    /// Preload all mushroom images. Images load asynchronously;
    /// get() will return the element once the browser has fetched it.
    pub fn preload_all(&mut self) {
        let keys = [
            "chanterelle",
            "fly-agaric",
            "king-bolete",
            "oyster",
            "shiitake",
            "turkey-tail",
            "honey-fungus",
            "chaga",
            "cordyceps",
            "morel",
            "death-cap",
            "reishi",
            "enoki",
            "lions-mane",
            "matsutake",
            "maitake",
            "destroying-angel",
            "porcini",
            "chicken-of-woods",
            "shaggy-ink-cap",
            "penny-bun",
            "giant-puffball",
            "jelly-ear",
            "birch-polypore",
            "false-morel",
            "wood-ear",
            "agarikon",
            "jack-o-lantern",
        ];

        for key in keys {
            let img = HtmlImageElement::new().expect("create img element");
            let src = format!("{}.jpg", key);
            img.set_src(&src);
            self.images.insert(key.to_owned(), img);
        }
    }

    /// Get a loaded image by key. Returns None if not yet loaded or key unknown.
    pub fn get(&self, key: &str) -> Option<&HtmlImageElement> {
        self.images.get(key).filter(|img| img.complete() && img.natural_width() > 0)
    }
}
