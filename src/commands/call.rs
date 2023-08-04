use serenity::{
    model::channel::Message,
    prelude::*,
};

pub async fn call(ctx: &Context, msg: &Message, args: Vec<&str>) {
    println!("hi"); 
}

