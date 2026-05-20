use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ImportOptions {
    pub user_login: String,
    pub start_date: String,
    pub end_date: String,
    pub download_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportedObservationRecord {
    pub record_id: String,
    pub common_name: String,
    pub scientific_name: String,
    pub observation_id: u64,
    pub observed_on: Option<String>,
    pub observer_login: String,
    pub observation_url: Option<String>,
    pub image_url: String,
    pub image_license: Option<String>,
    pub image_attribution: Option<String>,
    pub downloaded_image_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ObservationSearchResponse {
    pub total_results: usize,
    pub per_page: usize,
    pub results: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
pub struct Observation {
    pub id: u64,
    pub species_guess: Option<String>,
    pub observed_on: Option<String>,
    pub uri: Option<String>,
    pub license_code: Option<String>,
    pub user: Option<ObservationUser>,
    pub taxon: Option<ObservationTaxon>,
    #[serde(default)]
    pub photos: Vec<ObservationPhoto>,
}

#[derive(Debug, Deserialize)]
pub struct ObservationUser {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct ObservationTaxon {
    pub name: String,
    pub preferred_common_name: Option<String>,
    pub iconic_taxon_name: Option<String>,
    pub default_photo: Option<ObservationPhoto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ObservationPhoto {
    pub url: String,
    pub medium_url: Option<String>,
    pub attribution: Option<String>,
    pub license_code: Option<String>,
}
