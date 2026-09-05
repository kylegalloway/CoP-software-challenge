use rusqlite::{
    Error, ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, Value, ValueRef},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryStatus {
    Pending,
    Confirmed,
    Rejected,
}

impl FromSql for EntryStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| match s {
            "pending" => Ok(EntryStatus::Pending),
            "confirmed" => Ok(EntryStatus::Confirmed),
            "rejected" => Ok(EntryStatus::Rejected),
            _ => Err(FromSqlError::InvalidType),
        })
    }
}

impl ToSql for EntryStatus {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, Error> {
        let s = match self {
            EntryStatus::Pending => "pending",
            EntryStatus::Confirmed => "confirmed",
            EntryStatus::Rejected => "rejected",
        };
        Ok(ToSqlOutput::Owned(Value::Text(s.to_string())))
    }
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
    pub needs_specific_year: bool,
    pub specific_year: Option<u16>,
    pub image_url: Option<String>,
    pub status: EntryStatus,
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
    pub needs_specific_year: bool,
    pub specific_year: Option<u16>,
    pub image_url: Option<String>,
    pub status: EntryStatus,
}
