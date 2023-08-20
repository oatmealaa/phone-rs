use sqlx::{migrate::MigrateDatabase, Sqlite};
use sqlx::*;

pub mod dbutils;

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn db_init() -> Result<()> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await.unwrap();
    }
    
    let mut conn = SqliteConnection::connect(DB_URL).await?;

    sqlx::query!("CREATE TABLE IF NOT EXISTS calls (channel_id TEXT NOT NULL, connection_id TEXT);")
        .execute(&mut conn)
        .await?;

    Ok(()) 
}

