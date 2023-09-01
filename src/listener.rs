 use serenity::{
     async_trait,
     model::{channel::Message, gateway::Ready, prelude::*},
     prelude::*,
 };
use crate::commands;
use tokio::task::spawn;
use crate::callhandler;

pub struct Handler;

const PREFIX: &str = "!";
const BOT_ID: u64 = 1137049881421299722;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == UserId(BOT_ID) {
            return;
        }

        if let Some(cid) = callhandler::get_connection(msg.channel_id).await {
            println!("{:?}", cid.as_u64());
            callhandler::send_message(&msg,&ctx,cid).await; 
        }

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
