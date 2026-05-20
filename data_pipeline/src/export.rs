use std::collections::HashSet;
use std::fs;
use std::path::Path;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::records::{ImportedObservationRecord, Observation, ObservationPhoto};

#[derive(Debug, Clone, Serialize)]
pub struct GameCardExport {
    pub card_id: String,
    pub record_id: String,
    pub display_name: String,
    pub scientific_name: String,
    pub observation_id: u64,
    pub image_url: String,
    pub observation_url: Option<String>,
    pub accepted_buckets: AcceptedBuckets,
    pub provenance: CardProvenance,
}

#[derive(Debug, Clone, Serialize)]
pub struct AcceptedBuckets {
    pub ecology: String,
    pub color: String,
    pub season: String,
    pub function: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CardProvenance {
    pub observer_login: String,
    pub observed_on: Option<String>,
    pub image_license: Option<String>,
    pub image_attribution: Option<String>,
    pub downloaded_image_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportReport {
    pub fetched_observation_count: usize,
    pub matched_species_count: usize,
    pub card_count: usize,
    pub playable: bool,
}

#[derive(Debug, Clone, Copy)]
struct CatalogReference {
    record_id: &'static str,
    scientific_name: &'static str,
    common_name: &'static str,
    accepted_buckets: [&'static str; 4],
    aliases: &'static [&'static str],
}

const CATALOG_REFERENCES: &[CatalogReference] = &[
    CatalogReference { record_id: "chanterelle", scientific_name: "Cantharellus cibarius", common_name: "Chanterelle", accepted_buckets: ["Mycorrhizal", "Yellow/Gold", "Summer", "Choice Edible"], aliases: &["Golden Chanterelle"] },
    CatalogReference { record_id: "fly-agaric", scientific_name: "Amanita muscaria", common_name: "Fly Agaric", accepted_buckets: ["Mycorrhizal", "Red/Orange", "Autumn", "Toxic"], aliases: &[] },
    CatalogReference { record_id: "king-bolete", scientific_name: "Boletus edulis", common_name: "King Bolete", accepted_buckets: ["Mycorrhizal", "Brown/Tan", "Summer", "Choice Edible"], aliases: &["Cep", "Porcino"] },
    CatalogReference { record_id: "oyster", scientific_name: "Pleurotus ostreatus", common_name: "Oyster Mushroom", accepted_buckets: ["Saprotrophic", "White/Cream", "Autumn", "Choice Edible"], aliases: &[] },
    CatalogReference { record_id: "shiitake", scientific_name: "Lentinula edodes", common_name: "Shiitake", accepted_buckets: ["Saprotrophic", "Brown/Tan", "Autumn", "Choice Edible"], aliases: &[] },
    CatalogReference { record_id: "turkey-tail", scientific_name: "Trametes versicolor", common_name: "Turkey Tail", accepted_buckets: ["Saprotrophic", "Brown/Tan", "Autumn", "Medicinal"], aliases: &[] },
    CatalogReference { record_id: "honey-fungus", scientific_name: "Armillaria mellea", common_name: "Honey Fungus", accepted_buckets: ["Parasitic", "Yellow/Gold", "Autumn", "Inedible"], aliases: &[] },
    CatalogReference { record_id: "chaga", scientific_name: "Inonotus obliquus", common_name: "Chaga", accepted_buckets: ["Parasitic", "Brown/Tan", "Winter", "Medicinal"], aliases: &[] },
    CatalogReference { record_id: "cordyceps", scientific_name: "Ophiocordyceps sinensis", common_name: "Cordyceps", accepted_buckets: ["Parasitic", "Red/Orange", "Summer", "Medicinal"], aliases: &["Cordyceps sinensis", "Cordyceps militaris"] },
    CatalogReference { record_id: "morel", scientific_name: "Morchella esculenta", common_name: "Morel", accepted_buckets: ["Mycorrhizal", "Brown/Tan", "Spring", "Choice Edible"], aliases: &["Yellow Morel"] },
    CatalogReference { record_id: "death-cap", scientific_name: "Amanita phalloides", common_name: "Death Cap", accepted_buckets: ["Endophytic", "White/Cream", "Autumn", "Toxic"], aliases: &[] },
    CatalogReference { record_id: "reishi", scientific_name: "Ganoderma lucidum", common_name: "Reishi", accepted_buckets: ["Saprotrophic", "Red/Orange", "Summer", "Medicinal"], aliases: &[] },
    CatalogReference { record_id: "enoki", scientific_name: "Flammulina velutipes", common_name: "Enoki", accepted_buckets: ["Saprotrophic", "White/Cream", "Winter", "Choice Edible"], aliases: &["Velvet Shank"] },
    CatalogReference { record_id: "lions-mane", scientific_name: "Hericium erinaceus", common_name: "Lion's Mane", accepted_buckets: ["Saprotrophic", "White/Cream", "Autumn", "Medicinal"], aliases: &["Lions Mane"] },
    CatalogReference { record_id: "matsutake", scientific_name: "Tricholoma matsutake", common_name: "Matsutake", accepted_buckets: ["Mycorrhizal", "White/Cream", "Autumn", "Choice Edible"], aliases: &[] },
    CatalogReference { record_id: "maitake", scientific_name: "Grifola frondosa", common_name: "Maitake", accepted_buckets: ["Parasitic", "Brown/Tan", "Autumn", "Medicinal"], aliases: &["Hen of the Woods"] },
    CatalogReference { record_id: "destroying-angel", scientific_name: "Amanita virosa", common_name: "Destroying Angel", accepted_buckets: ["Mycorrhizal", "White/Cream", "Summer", "Toxic"], aliases: &[] },
    CatalogReference { record_id: "porcini", scientific_name: "Imleria badia", common_name: "Bay Bolete", accepted_buckets: ["Mycorrhizal", "Brown/Tan", "Autumn", "Choice Edible"], aliases: &["Porcini"] },
    CatalogReference { record_id: "chicken-of-woods", scientific_name: "Laetiporus sulphureus", common_name: "Chicken of the Woods", accepted_buckets: ["Parasitic", "Yellow/Gold", "Summer", "Choice Edible"], aliases: &[] },
    CatalogReference { record_id: "shaggy-ink-cap", scientific_name: "Coprinus comatus", common_name: "Shaggy Ink Cap", accepted_buckets: ["Saprotrophic", "White/Cream", "Autumn", "Choice Edible"], aliases: &["Shaggy Mane"] },
    CatalogReference { record_id: "penny-bun", scientific_name: "Boletus edulis var.", common_name: "Penny Bun", accepted_buckets: ["Mycorrhizal", "Brown/Tan", "Autumn", "Choice Edible"], aliases: &[] },
    CatalogReference { record_id: "giant-puffball", scientific_name: "Calvatia gigantea", common_name: "Giant Puffball", accepted_buckets: ["Saprotrophic", "White/Cream", "Autumn", "Choice Edible"], aliases: &[] },
    CatalogReference { record_id: "jelly-ear", scientific_name: "Auricularia auricula-judae", common_name: "Jelly Ear", accepted_buckets: ["Saprotrophic", "Brown/Tan", "Winter", "Inedible"], aliases: &["Jew's Ear", "Jews Ear"] },
    CatalogReference { record_id: "birch-polypore", scientific_name: "Fomitopsis betulina", common_name: "Birch Polypore", accepted_buckets: ["Parasitic", "White/Cream", "Autumn", "Medicinal"], aliases: &["Piptoporus betulinus"] },
    CatalogReference { record_id: "false-morel", scientific_name: "Gyromitra esculenta", common_name: "False Morel", accepted_buckets: ["Mycorrhizal", "Brown/Tan", "Spring", "Toxic"], aliases: &[] },
    CatalogReference { record_id: "wood-ear", scientific_name: "Auricularia polytricha", common_name: "Wood Ear", accepted_buckets: ["Saprotrophic", "Brown/Tan", "Autumn", "Choice Edible"], aliases: &[] },
    CatalogReference { record_id: "agarikon", scientific_name: "Laricifomes officinalis", common_name: "Agarikon", accepted_buckets: ["Parasitic", "White/Cream", "Summer", "Medicinal"], aliases: &["Fomitopsis officinalis"] },
    CatalogReference { record_id: "jack-o-lantern", scientific_name: "Omphalotus olearius", common_name: "Jack O'Lantern", accepted_buckets: ["Saprotrophic", "Red/Orange", "Autumn", "Toxic"], aliases: &["Jack O Lantern"] },
];

pub fn normalize_records(
    observations: &[Observation],
    download_dir: Option<&str>,
) -> Result<Vec<ImportedObservationRecord>, String> {
    let client = Client::builder()
        .user_agent("sporefall-data-pipeline/0.1")
        .build()
        .map_err(|error| format!("Failed to create download client: {error}"))?;

    let mut seen_species = HashSet::new();
    let mut records = Vec::new();

    for observation in observations {
        let Some(taxon) = observation.taxon.as_ref() else {
            continue;
        };
        if taxon.iconic_taxon_name.as_deref() != Some("Fungi") {
            continue;
        }

        let Some(reference) = match_reference(
            &taxon.name,
            taxon.preferred_common_name
                .as_deref()
                .or(observation.species_guess.as_deref()),
        ) else {
            continue;
        };

        if !seen_species.insert(reference.record_id) {
            continue;
        }

        let Some(photo) = observation.photos.first().or(taxon.default_photo.as_ref()) else {
            continue;
        };
        let image_url = preferred_image_url(photo);
        let downloaded_image_path = if let Some(path) = download_dir {
            Some(download_image(&client, path, reference.record_id, observation.id, &image_url)?)
        } else {
            None
        };

        records.push(ImportedObservationRecord {
            record_id: reference.record_id.to_owned(),
            common_name: taxon
                .preferred_common_name
                .clone()
                .unwrap_or_else(|| reference.common_name.to_owned()),
            scientific_name: taxon.name.clone(),
            observation_id: observation.id,
            observed_on: observation.observed_on.clone(),
            observer_login: observation
                .user
                .as_ref()
                .map(|user| user.login.clone())
                .unwrap_or_default(),
            observation_url: observation.uri.clone(),
            image_url,
            image_license: photo
                .license_code
                .clone()
                .or_else(|| observation.license_code.clone()),
            image_attribution: photo.attribution.clone(),
            downloaded_image_path,
        });

        if records.len() >= 28 {
            break;
        }
    }

    Ok(records)
}

pub fn export_game_cards(records: &[ImportedObservationRecord]) -> Vec<GameCardExport> {
    records
        .iter()
        .filter_map(|record| {
            let reference = match_reference(&record.scientific_name, Some(&record.common_name))?;
            Some(GameCardExport {
                card_id: format!("inat-card-{}", record.observation_id),
                record_id: record.record_id.clone(),
                display_name: record.common_name.clone(),
                scientific_name: record.scientific_name.clone(),
                observation_id: record.observation_id,
                image_url: record.image_url.clone(),
                observation_url: record.observation_url.clone(),
                accepted_buckets: AcceptedBuckets {
                    ecology: reference.accepted_buckets[0].to_owned(),
                    color: reference.accepted_buckets[1].to_owned(),
                    season: reference.accepted_buckets[2].to_owned(),
                    function: reference.accepted_buckets[3].to_owned(),
                },
                provenance: CardProvenance {
                    observer_login: record.observer_login.clone(),
                    observed_on: record.observed_on.clone(),
                    image_license: record.image_license.clone(),
                    image_attribution: record.image_attribution.clone(),
                    downloaded_image_path: record.downloaded_image_path.clone(),
                },
            })
        })
        .collect()
}

pub fn build_report(
    fetched_observation_count: usize,
    records: &[ImportedObservationRecord],
    cards: &[GameCardExport],
) -> ExportReport {
    ExportReport {
        fetched_observation_count,
        matched_species_count: records.len(),
        card_count: cards.len(),
        playable: !cards.is_empty(),
    }
}

fn match_reference(
    scientific_name: &str,
    common_name: Option<&str>,
) -> Option<&'static CatalogReference> {
    let scientific_key = normalize_name(scientific_name);
    let common_key = common_name.map(normalize_name);

    CATALOG_REFERENCES.iter().find(|reference| {
        let mut keys = vec![
            normalize_name(reference.scientific_name),
            normalize_name(reference.common_name),
        ];
        keys.extend(reference.aliases.iter().map(|alias| normalize_name(alias)));
        keys.into_iter().any(|candidate| {
            candidate == scientific_key
                || common_key
                    .as_ref()
                    .map(|value| candidate == *value)
                    .unwrap_or(false)
        })
    })
}

fn normalize_name(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'A'..='Z' => ch.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' => ch,
            _ => ' ',
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn preferred_image_url(photo: &ObservationPhoto) -> String {
    photo
        .medium_url
        .clone()
        .unwrap_or_else(|| photo.url.replace("/square.", "/medium."))
}

fn download_image(
    client: &Client,
    directory: &str,
    record_id: &str,
    observation_id: u64,
    image_url: &str,
) -> Result<String, String> {
    fs::create_dir_all(directory)
        .map_err(|error| format!("Failed to create download directory: {error}"))?;

    let extension = Path::new(image_url)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("jpg");
    let file_path = format!("{directory}\\{}-{}.{}", record_id, observation_id, extension);
    let bytes = client
        .get(image_url)
        .send()
        .map_err(|error| format!("Failed to download image: {error}"))?
        .bytes()
        .map_err(|error| format!("Failed to read image bytes: {error}"))?;
    fs::write(&file_path, bytes).map_err(|error| format!("Failed to write image file: {error}"))?;
    Ok(file_path)
}
