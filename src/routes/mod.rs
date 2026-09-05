use axum::Router;
use axum::routing::get;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub mod dates;

pub fn router(state: Arc<Mutex<Connection>>) -> Router {
    Router::new()
        .route("/api/date/{mm-dd}", get(dates::get_date))
        .with_state(state)
}
