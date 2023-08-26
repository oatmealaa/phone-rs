use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
};

use sqlx::{
    Sqlite,
    Connection,
    SqliteConnection,
    *,
};

use chrono::prelude::*;
use serenity::futures::TryStreamExt;
use tokio::time::interval;
use std::time::Duration;

use crate::commands::call::Call;

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn start() {
    println!("start");
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();
    let mut interval = interval(Duration::from_millis(100));
    
    sqlx::query!("DELETE FROM calls;").execute(&mut conn).await;

    loop {
        interval.tick().await;
        handlecalls().await;
    }
}

pub async fn handlecalls() {
    println!("calls");
    let mut pending: Vec<Call> = getpending().await;
    
    if pending.len() <= 1 {
        return;
    }

    let mut i: usize = 1;
    while i < pending.len() {
        connect(pending.get(i).unwrap(), pending.get(i-1).unwrap());
        
        i=i+2;
    }
}

pub async fn getpending() -> Vec<Call> {
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();
    
    let mut result = sqlx::query!("SELECT channel_id FROM calls WHERE connection_id IS NULL").fetch(&mut conn);

    let mut rows = vec![];
    while let Ok(Some(row)) = result.try_next().await {
        rows.push(Call {
            channel_id: ChannelId(row.channel_id.parse::<u64>().unwrap()),
            connection_id: None,
        });
    }

    rows
}

pub async fn connect(channel1: &Call, channel2: &Call) {
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();
    
    let (cid, cid2) = (channel1.channel_id.to_string(), channel2.channel_id.to_string());

    sqlx::query!("UPDATE calls SET connection_id = $1 WHERE channel_id = $2",cid2,cid)
    .execute(&mut conn).await;

    sqlx::query!("UPDATE calls SET connection_id = $1 WHERE channel_id = $2",cid,cid2)
    .execute(&mut conn).await;
   
} 

