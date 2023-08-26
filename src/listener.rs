 use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready},
     prelude::*,
 };
use crate::commands;
use tokio::task::spawn;
use crate::callhandler;

pub struct Handler;

const PREFIX: &str = "!";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        if !msg.content.starts_with(PREFIX) {
            return;
        }
        println!("command");
        commands::command(&ctx,&msg).await;
    }


    async fn ready(&self, ctx: Context, ready: Ready) {
        let thread = spawn(callhandler::start());
         
        println!("ready");
    }
}
