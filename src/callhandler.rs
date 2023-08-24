use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
};

use sqlx::{
    Sqlite,
    Connection,
    SqliteConnection,
};

use crate::commands::call::Call;

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn start() {
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();
    
    sqlx::query!("DELETE FROM calls;").execute(&mut conn).await;
}

pub async fn handlecalls() {
    let mut pending = getpending().await;
       
    let i: isize = 1;
    while i < pending.len() {
        connect(pending.get(i).unwrap(), pending.get(i-1).unwrap());
        
        i=i+2;
    }
}

pub async fn getpending() -> Vec<Call> {
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();
    
    let mut result = sqlx::query!("SELECT channel_id FROM calls WHERE connection_id IS NULL").execute(&mut conn).await;

    let mut rows = vec![];
    while let Ok(Some(row)) = result.try_next().await {
        rows.push(Call {
            channel_id: row.channel_id,
            connection_id: None,
        });
    }

    rows
}

pub async fn connect(channel1: Call, channel2: Call) {
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();
    
    sqlx::query!("UPDATE calls SET connection_id = $1 WHERE channel_id = $2",channel2.channel_id,channel1.channel_id)
    .execute(&mut conn).await;

    sqlx::query!("UPDATE calls SET connection_id = $1 WHERE channel_id = $2",channel1.channel_id,channel2.channel_id)
    .execute(&mut conn).await;
   
} 

