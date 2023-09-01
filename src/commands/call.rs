use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
};

use sqlx::{Sqlite, Connection, *};

use crate::utils::*;
use rand::prelude::*;
use crate::callhandler;

const DB_URL: &str = "sqlite://sqlite.db";

pub struct Call {
    pub channel_id: ChannelId,
    pub connection_id: Option<u64>,
}


pub async fn call(ctx: &Context, msg: &Message) {
    if let Some(cid) = callhandler::get_connection(msg.channel_id).await {
        msg.reply(&ctx, "Already connected to a call").await;
        return;
    }

    let call = Call::new(msg.channel_id).await;
    call.insert().await;
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
            
        let id = format!("{:?}",self.channel_id.as_u64());
        sqlx::query!("INSERT INTO calls (channel_id) VALUES ($1)",id)
        .execute(&mut conn)
        .await.unwrap();
    }
}


