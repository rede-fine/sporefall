use serde::Serialize;

use crate::records::NormalizedRecord;

#[derive(Debug, Clone, Serialize)]
pub struct GameCardExport {
    pub card_id: String,
    pub record_id: String,
    pub accepted_buckets: AcceptedBuckets,
}

#[derive(Debug, Clone, Serialize)]
pub struct AcceptedBuckets {
    pub ecology: String,
    pub taxonomy: String,
    pub function: String,
    pub season: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportReport {
    pub record_count: usize,
    pub card_count: usize,
    pub playable: bool,
}

pub fn export_game_cards(records: &[NormalizedRecord]) -> Vec<GameCardExport> {
    records
        .iter()
        .map(|record| GameCardExport {
            card_id: format!("card-{}", record.record_id),
            record_id: record.record_id.clone(),
            accepted_buckets: AcceptedBuckets {
                ecology: record.ecological_guild.clone(),
                taxonomy: record.taxonomic_group.clone(),
                function: record.functional_group.clone(),
                season: record.season_window.first().cloned().unwrap_or_default(),
                color: record.color_group.clone(),
            },
        })
        .collect()
}

pub fn build_report(records: &[NormalizedRecord], cards: &[GameCardExport]) -> ExportReport {
    ExportReport {
        record_count: records.len(),
        card_count: cards.len(),
        playable: !records.is_empty() && records.len() == cards.len(),
    }
}
