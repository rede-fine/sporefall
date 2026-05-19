use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ProvenanceNote {
    pub source: String,
}

impl ProvenanceNote {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceRecord {
    pub record_id: String,
    pub scientific_name: String,
    pub common_name: String,
    pub region: String,
    pub ecological_guild: String,
    pub taxonomic_group: String,
    pub functional_group: String,
    pub season_window: Vec<String>,
    pub color_group: String,
    pub image_asset_id: String,
    pub image_license: String,
    pub image_attribution: String,
    pub source_provenance: Vec<ProvenanceNote>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NormalizedRecord {
    pub record_id: String,
    pub scientific_name: String,
    pub common_name: String,
    pub region: String,
    pub ecological_guild: String,
    pub taxonomic_group: String,
    pub functional_group: String,
    pub season_window: Vec<String>,
    pub color_group: String,
    pub image_asset_id: String,
    pub image_license: String,
    pub image_attribution: String,
    pub source_provenance: Vec<ProvenanceNote>,
}

impl SourceRecord {
    pub fn normalize(self) -> NormalizedRecord {
        NormalizedRecord {
            record_id: self.record_id,
            scientific_name: self.scientific_name,
            common_name: self.common_name,
            region: self.region,
            ecological_guild: self.ecological_guild,
            taxonomic_group: self.taxonomic_group,
            functional_group: self.functional_group,
            season_window: self.season_window,
            color_group: self.color_group,
            image_asset_id: self.image_asset_id,
            image_license: self.image_license,
            image_attribution: self.image_attribution,
            source_provenance: self.source_provenance,
        }
    }
}
