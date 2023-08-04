 use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready},
     prelude::*,
 };
use crate::commands;

pub struct Handler;

const PREFIX: &str = "!";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        if !msg.content.starts_with(PREFIX) {
            return;
        }
        commands::command(&ctx,&msg).await;
    }


    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("ready");
    }
}
