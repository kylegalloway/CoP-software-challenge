use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rusqlite::{Connection, Error, Result};
use serde::Serialize;
use std::sync::{Arc, Mutex};

use crate::models::Entry;

#[derive(Serialize)]
pub struct DateResponse {
    pub date: String,
    entries: Vec<Entry>,
    stats: Stats,
}

#[derive(Serialize)]
pub struct Stats {
    global_total_confirmed_entries: i64,
}

pub async fn get_date(
    State(conn): State<Arc<Mutex<Connection>>>,
    Path(mm_dd): Path<String>,
) -> Result<Json<DateResponse>, StatusCode> {
    // 1. Parse "mm-dd" -> (u8, u8). Split on '-', parse both halves as u8.
    let (month, day) = parse_mm_dd(&mm_dd).ok_or(StatusCode::BAD_REQUEST)?;

    // 2. Bounds-checks: month in 1..=12, day in 1..=31 (don't care to check Feb. 31st).
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 3. Query confirmed entries for (month, day), ordered however you like
    //    (net votes desc once Phase 6 exists; created_at for now).
    //    SELECT id, title, quote, source_url, secondary_source_url,
    //           image_url, specific_year
    //    FROM entries
    //    WHERE month = ?1 AND day = ?2 AND status = 'confirmed'
    let entries =
        query_entries_for_date(&conn, month, day).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 4. Sitewide stat — a separate cheap query, not per-date:
    //    SELECT COUNT(*) FROM entries WHERE status = 'confirmed'
    let total = count_confirmed(&conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DateResponse {
        date: format!("{:02}-{:02}", month, day),
        entries,
        stats: Stats {
            global_total_confirmed_entries: total,
        },
    }))
}

fn parse_mm_dd(s: &str) -> Option<(u8, u8)> {
    let (m, d) = s.split_once('-')?;
    Some((m.parse().ok()?, d.parse().ok()?))
}

fn query_entries_for_date(
    conn: &Arc<Mutex<Connection>>,
    month: u8,
    day: u8,
) -> Result<Vec<Entry>, Error> {
    let conn = conn.lock().unwrap();

    let mut stmt = conn.prepare(
        "SELECT id, month, day, title, quote, source_url, secondary_source_url,
            needs_secondary_source, needs_specific_year, specific_year,
            image_url, status, created_at
            FROM entries
            WHERE month = ?1 AND day = ?2 AND status = 'confirmed'
            ORDER BY created_at DESC",
    )?;

    let rows = stmt.query_map(rusqlite::params![month, day], |row| {
        Ok(Entry {
            id: row.get(0)?,
            month: row.get(1)?,
            day: row.get(2)?,
            title: row.get(3)?,
            quote: row.get(4)?,
            source_url: row.get(5)?,
            secondary_source_url: row.get(6)?,
            needs_secondary_source: row.get(7)?,
            needs_specific_year: row.get(8)?,
            specific_year: row.get(9)?,
            image_url: row.get(10)?,
            status: row.get(11)?, // see note below on EntryStatus
            created_at: row.get(12)?,
        })
    })?;

    rows.collect()
}

fn count_confirmed(conn: &Arc<Mutex<Connection>>) -> Result<i64, Error> {
    let conn = conn.lock().unwrap();

    conn.query_row(
        "SELECT COUNT(*) FROM entries WHERE status = 'confirmed'",
        [],
        |row| row.get(0),
    )
}
