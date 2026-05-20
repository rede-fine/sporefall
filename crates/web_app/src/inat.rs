use std::collections::HashSet;

use gloo_net::http::Request;
use serde::Deserialize;

use crate::catalog::{find_catalog_entry, CatalogProvenance, RuntimeCatalogEntry};

const FUNGI_TAXON_ID: u32 = 47170;
const MAX_IMPORT_SPECIES: usize = 28;
const PAGE_SIZE: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportRequest {
    pub user_login: String,
    pub start_date: String,
    pub end_date: String,
}

impl ImportRequest {
    pub fn validate(&self) -> Result<(), String> {
        let user_login = self.user_login.trim();
        let start_date = self.start_date.trim();
        let end_date = self.end_date.trim();

        if user_login.is_empty() {
            return Err("Enter an iNaturalist username.".to_owned());
        }
        if !looks_like_date(start_date) || !looks_like_date(end_date) {
            return Err("Choose both start and end dates.".to_owned());
        }
        if start_date > end_date {
            return Err("The start date must be on or before the end date.".to_owned());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportSummary {
    pub user_login: String,
    pub start_date: String,
    pub end_date: String,
    pub fetched_observations: usize,
    pub matched_species_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedCatalog {
    pub entries: Vec<RuntimeCatalogEntry>,
    pub summary: ImportSummary,
}

pub async fn import_catalog(request: ImportRequest) -> Result<ImportedCatalog, String> {
    request.validate()?;

    let mut entries = Vec::new();
    let mut seen_species = HashSet::new();
    let mut fetched_observations = 0usize;
    let mut page = 1usize;
    let normalized_login = request.user_login.trim().to_owned();

    loop {
        let url = format!(
            "https://api.inaturalist.org/v1/observations?user_login={}&taxon_id={}&photos=true&d1={}&d2={}&order_by=observed_on&order=desc&per_page={}&page={}",
            encode_component(&normalized_login),
            FUNGI_TAXON_ID,
            encode_component(request.start_date.trim()),
            encode_component(request.end_date.trim()),
            PAGE_SIZE,
            page
        );

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|error| format!("Could not reach iNaturalist: {error}"))?;

        if !response.ok() {
            return Err(match response.status() {
                404 => "The iNaturalist account could not be found.".to_owned(),
                422 => "iNaturalist could not process that import request.".to_owned(),
                status => format!("iNaturalist returned HTTP {status}."),
            });
        }

        let payload: ObservationSearchResponse = response
            .json()
            .await
            .map_err(|error| format!("Could not read the iNaturalist response: {error}"))?;

        if payload.results.is_empty() {
            break;
        }

        fetched_observations += payload.results.len();

        for observation in payload.results {
            if let Some(entry) = observation_to_entry(&observation) {
                if seen_species.insert(entry.id.clone()) {
                    entries.push(entry);
                }
            }

            if entries.len() >= MAX_IMPORT_SPECIES {
                break;
            }
        }

        if entries.len() >= MAX_IMPORT_SPECIES
            || page * payload.per_page >= payload.total_results
        {
            break;
        }

        page += 1;
    }

    if entries.is_empty() {
        return Err(
            "No supported fungi observations matched Sporefall's catalog. Try a wider date range."
                .to_owned(),
        );
    }

    let summary = ImportSummary {
        user_login: normalized_login,
        start_date: request.start_date.trim().to_owned(),
        end_date: request.end_date.trim().to_owned(),
        fetched_observations,
        matched_species_count: entries.len(),
    };

    Ok(ImportedCatalog { entries, summary })
}

fn observation_to_entry(observation: &Observation) -> Option<RuntimeCatalogEntry> {
    let taxon = observation.taxon.as_ref()?;
    if taxon.iconic_taxon_name.as_deref() != Some("Fungi") {
        return None;
    }

    let catalog_entry = find_catalog_entry(
        &taxon.name,
        taxon.preferred_common_name
            .as_deref()
            .or(observation.species_guess.as_deref()),
    )?;

    let photo = observation
        .photos
        .first()
        .or(taxon.default_photo.as_ref())?;
    let image_url = photo
        .medium_url
        .clone()
        .unwrap_or_else(|| promote_photo_url(&photo.url));

    Some(RuntimeCatalogEntry {
        id: catalog_entry.id.to_owned(),
        display_name: taxon
            .preferred_common_name
            .clone()
            .or_else(|| observation.species_guess.clone())
            .unwrap_or_else(|| catalog_entry.display_name.to_owned()),
        latin_name: taxon.name.clone(),
        image_key: format!("inat-{}-{}", catalog_entry.id, observation.id),
        targets: catalog_entry.targets,
        provenance: CatalogProvenance {
            source_name: "iNaturalist".to_owned(),
            source_url: observation.uri.clone(),
            image_url: Some(image_url),
            image_license: photo
                .license_code
                .clone()
                .or_else(|| observation.license_code.clone()),
            image_attribution: photo.attribution.clone(),
            observed_on: observation.observed_on.clone(),
            observer_login: observation.user.as_ref().map(|user| user.login.clone()),
        },
    })
}

fn looks_like_date(value: &str) -> bool {
    value.len() == 10
        && value.chars().enumerate().all(|(index, ch)| match index {
            4 | 7 => ch == '-',
            _ => ch.is_ascii_digit(),
        })
}

fn encode_component(value: &str) -> String {
    js_sys::encode_uri_component(value)
        .as_string()
        .unwrap_or_else(|| value.to_owned())
}

fn promote_photo_url(url: &str) -> String {
    url.replace("/square.", "/medium.")
}

#[derive(Debug, Deserialize)]
struct ObservationSearchResponse {
    total_results: usize,
    per_page: usize,
    results: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
struct Observation {
    id: u64,
    species_guess: Option<String>,
    observed_on: Option<String>,
    uri: Option<String>,
    license_code: Option<String>,
    user: Option<ObservationUser>,
    taxon: Option<ObservationTaxon>,
    #[serde(default)]
    photos: Vec<ObservationPhoto>,
}

#[derive(Debug, Deserialize)]
struct ObservationUser {
    login: String,
}

#[derive(Debug, Deserialize)]
struct ObservationTaxon {
    name: String,
    preferred_common_name: Option<String>,
    iconic_taxon_name: Option<String>,
    default_photo: Option<ObservationPhoto>,
}

#[derive(Debug, Deserialize)]
struct ObservationPhoto {
    url: String,
    medium_url: Option<String>,
    attribution: Option<String>,
    license_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{looks_like_date, promote_photo_url};

    #[test]
    fn validates_iso_dates() {
        assert!(looks_like_date("2026-05-20"));
        assert!(!looks_like_date("20-05-2026"));
    }

    #[test]
    fn upgrades_square_photo_urls() {
        assert_eq!(
            promote_photo_url("https://example.com/photos/1/square.jpeg"),
            "https://example.com/photos/1/medium.jpeg"
        );
    }
}
