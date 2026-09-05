mod db;
mod models;

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
}
