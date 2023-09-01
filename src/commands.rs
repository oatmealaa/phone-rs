use serenity::{
    model::channel::Message,
    prelude::*,
};

pub mod call;
use crate::commands::{call::*,hangup::*};

pub mod hangup;

pub async fn command(ctx: &Context, msg: &Message) {
    let split: Vec<&str> = msg.content.split(" ").collect();
    


    match msg.content.as_str() {
        "!call" => call(ctx,msg).await,
        "!hangup" => hangup(ctx,msg).await,

        _ => (),
    }

}
