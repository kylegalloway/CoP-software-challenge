use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryStatus {
    Pending,
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: i64,
    pub month: u8,
    pub day: u8,
    pub title: String,
    pub quote: String,
    pub source_url: String,
    pub secondary_source_url: Option<String>,
    pub needs_secondary_source: bool,
    pub image_url: Option<String>,
    pub status: EntryStatus,
    pub submitted_by: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEntry {
    pub month: u8,
    pub day: u8,
    pub title: String,
    pub quote: String,
    pub source_url: String,
    pub secondary_source_url: Option<String>,
    pub needs_secondary_source: bool,
    pub image_url: Option<String>,
    pub submitted_by: Option<String>,
}
