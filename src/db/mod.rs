use crate::models::NewEntry;
use rusqlite::Connection;

pub fn open(path: &str) -> rusqlite::Result<Connection> {
    // Open the connection
    let conn = Connection::open(path)?;

    // Ensure the foreign key constraints are on
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    // Setup the database schema
    conn.execute_batch(include_str!("schema.sql"))?;

    // Seed the database if it's empty
    seed_if_empty(&conn)?;

    Ok(conn)
}

fn seed_if_empty(conn: &Connection) -> rusqlite::Result<()> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM entries", [], |row| row.get(0))?;
    if count == 0 {
        let raw_entry_data = include_str!("../../data/seed/dates.seed.json");
        let seed_entries: Vec<NewEntry> =
            serde_json::from_str(raw_entry_data).expect("Failed to parse seed data");

        for entry in &seed_entries {
            conn.execute (
            "INSERT INTO entries (month, day, title, quote, source_url, secondary_source_url, needs_secondary_source, needs_specific_year, specific_year, image_url, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                entry.month,
                entry.day,
                entry.title,
                entry.quote,
                entry.source_url,
                entry.secondary_source_url,
                entry.needs_secondary_source,
                entry.needs_specific_year,
                entry.specific_year,
                entry.image_url,
                entry.status,
            ],
        )?;
        }
    }
    Ok(())
}
