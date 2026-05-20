use reqwest::blocking::Client;

use crate::records::{ImportOptions, Observation, ObservationSearchResponse};

const FUNGI_TAXON_ID: u32 = 47170;
const PAGE_SIZE: usize = 100;
const MAX_PAGES: usize = 5;

pub fn fetch_observations(options: &ImportOptions) -> Result<Vec<Observation>, String> {
    validate_options(options)?;

    let client = Client::builder()
        .user_agent("sporefall-data-pipeline/0.1")
        .build()
        .map_err(|error| format!("Failed to create HTTP client: {error}"))?;

    let mut observations = Vec::new();
    let mut page = 1usize;

    loop {
        let taxon_id = FUNGI_TAXON_ID.to_string();
        let per_page = PAGE_SIZE.to_string();
        let page_value = page.to_string();
        let response = client
            .get("https://api.inaturalist.org/v1/observations")
            .query(&[
                ("user_login", options.user_login.trim()),
                ("taxon_id", taxon_id.as_str()),
                ("photos", "true"),
                ("d1", options.start_date.trim()),
                ("d2", options.end_date.trim()),
                ("order_by", "observed_on"),
                ("order", "desc"),
                ("per_page", per_page.as_str()),
                ("page", page_value.as_str()),
            ])
            .send()
            .map_err(|error| format!("Failed to fetch observations: {error}"))?;

        if !response.status().is_success() {
            return Err(format!(
                "iNaturalist observations request failed with status {}.",
                response.status()
            ));
        }

        let payload: ObservationSearchResponse = response
            .json()
            .map_err(|error| format!("Failed to parse iNaturalist response: {error}"))?;

        if payload.results.is_empty() {
            break;
        }

        observations.extend(payload.results);

        if page * payload.per_page >= payload.total_results || page >= MAX_PAGES {
            break;
        }

        page += 1;
    }

    Ok(observations)
}

fn validate_options(options: &ImportOptions) -> Result<(), String> {
    if options.user_login.trim().is_empty() {
        return Err("Missing --user-login.".to_owned());
    }
    if !looks_like_date(&options.start_date) || !looks_like_date(&options.end_date) {
        return Err("Expected ISO dates for --start-date and --end-date.".to_owned());
    }
    if options.start_date > options.end_date {
        return Err("--start-date must be on or before --end-date.".to_owned());
    }
    Ok(())
}

fn looks_like_date(value: &str) -> bool {
    value.len() == 10
        && value.chars().enumerate().all(|(index, ch)| match index {
            4 | 7 => ch == '-',
            _ => ch.is_ascii_digit(),
        })
}
