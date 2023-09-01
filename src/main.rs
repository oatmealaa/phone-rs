use std::env;

use serenity::{
    prelude::*,
};

pub mod utils;

pub mod commands;

pub mod listener;
use crate::listener::*;

pub mod db;
use crate::db::*;

pub mod callhandler;
use crate::callhandler::start;

pub mod log;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let intents = GatewayIntents::all();
    
let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    
    db_init().await.unwrap();
   if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
