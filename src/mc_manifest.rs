use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct McManifestVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub snapshot_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub sha1: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: u8,
}

#[derive(Debug, Deserialize)]
pub struct McManifestLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct McManifest {
    pub latest: McManifestLatest,
    pub versions: Vec<McManifestVersion>,
}