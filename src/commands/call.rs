use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
};

use sqlx::{Sqlite, Connection, *};

use crate::utils::*;
use rand::prelude::*;

const DB_URL: &str = "sqlite://sqlite.db";

pub struct Call {
    pub channel_id: ChannelId,
    pub connection_id: Option<u64>,
}


pub async fn call(ctx: &Context, msg: &Message, args: Vec<&str>) {
    //let channel_id = ChannelId(parse::<u64>(args[1]));
    println!("call");
    parse_channelid(args.get(1).unwrap()).await.unwrap().say(&ctx, "heyyyyyyyyyyyyyyy").await.unwrap();
    println!("what");
}

impl Call {
    async fn new(channel_id: ChannelId) -> Self {
        let connection_id = None;
        Self {
            channel_id,
            connection_id,

        }
    }
    async fn insert(&self) {
        let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();
            
        let id = String(self.channel_id).unwrap;

        sqlx::query!("INSERT INTO calls (channel_id) VALUES ($1)",id)
        .execute(&mut conn)
        .await.unwrap();
    }
}


