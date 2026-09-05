use std::sync::{Arc, Mutex};

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // Pull DB_PATH from environment variable or use default
    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "data/database.db".to_string());

    // Connect to the db
    let conn = db::open(&db_path).expect("Failed to open database");

    // Verify connection works
    println!("Database opened successfully at path: {}", db_path);
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM entries", [], |row| row.get(0))
        .expect("Failed to count entries in database");
    println!("Database currently contains {} rows.", count);

    // wrap the connection in an Arc to share it across threads
    let state = Arc::new(Mutex::new(conn));

    // Add fallback service to serve static files if needed
    let app = routes::router(state).fallback_service(tower_http::services::ServeDir::new("web"));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Server running on http://{}", address);
    axum::serve(listener, app).await.unwrap();
}
