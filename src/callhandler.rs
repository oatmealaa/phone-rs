use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
    model::channel::Message,

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
        handle_calls().await;
    }
}

pub async fn handle_calls() {
    let pending: Vec<Call> = get_pending().await;
    
    if pending.len() <= 1 {
        return;
    }
    let mut i: usize = 1;
    while i < pending.len() {
        connect(pending.get(i).unwrap(), pending.get(i-1).unwrap()).await;
        i=i+2;
    }
}

pub async fn get_pending() -> Vec<Call> {
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

    println!("connected");
} 

pub async fn get_connection(channel_id: ChannelId) -> Option<ChannelId> {
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();

    let cid = format!("{:?}", channel_id.as_u64());

    let result = sqlx::query!("SELECT connection_id FROM calls WHERE channel_id =  $1", cid).fetch_one(&mut conn).await;

    if let Ok(row) = result {
        if let Some(id) = row.connection_id {
            return Some(ChannelId(id.parse::<u64>().unwrap()));
        }


    } 
    None
}

pub async fn send_message(msg: &Message, ctx: &Context, connected_channel: ChannelId) {
    connected_channel.say(&ctx ,format!("{}: {}", msg.author.name, msg.content)).await.unwrap();
}

pub async fn end_call(channel1: ChannelId, channel2: ChannelId) {
    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();

    let (cid, cid2) = (channel1.to_string(), channel2.to_string());

    sqlx::query!("
    UPDATE calls SET connection_id = NULL WHERE channel_id = $1;
    UPDATE calls SET connection_id = NULL WHERE channel_id = $2;
    ", cid, cid2).execute(&mut conn).await;
}

