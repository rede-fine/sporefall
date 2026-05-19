mod export;
mod records;

use export::{build_report, export_game_cards};
use records::{NormalizedRecord, ProvenanceNote, SourceRecord};

fn main() {
    let source_records = vec![SourceRecord {
        record_id: "amanita-muscaria".to_owned(),
        scientific_name: "Amanita muscaria".to_owned(),
        common_name: "Fly Agaric".to_owned(),
        region: "North America".to_owned(),
        ecological_guild: "Mycorrhizal".to_owned(),
        taxonomic_group: "Agarics".to_owned(),
        functional_group: "Toxic".to_owned(),
        season_window: vec!["Autumn".to_owned()],
        color_group: "Red".to_owned(),
        image_asset_id: "commons-fly-agaric-1".to_owned(),
        image_license: "CC BY-SA 4.0".to_owned(),
        image_attribution: "Wikimedia Commons contributor".to_owned(),
        source_provenance: vec![
            ProvenanceNote::new("Catalogue of Life"),
            ProvenanceNote::new("MyCoPortal"),
            ProvenanceNote::new("Wikimedia Commons"),
        ],
    }];

    let normalized: Vec<NormalizedRecord> = source_records
        .into_iter()
        .map(SourceRecord::normalize)
        .collect();
    let cards = export_game_cards(&normalized);
    let report = build_report(&normalized, &cards);

    let payload = serde_json::json!({
        "records": normalized,
        "cards": cards,
        "report": report,
    });

    println!("{}", serde_json::to_string_pretty(&payload).expect("serializes pipeline output"));
}
