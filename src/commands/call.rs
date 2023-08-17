use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
};

use crate::utils::*;
use rand::prelude::*;


pub struct Call {
    pub channel_id: ChannelId,
    pub id: u64,
    pub connection_id: Option<u64>,
}


pub async fn call(ctx: &Context, msg: &Message, args: Vec<&str>) {
    //let channel_id = ChannelId(parse::<u64>(args[1]));
    println!("call");
    parse_channelid(args.get(1).unwrap()).await.unwrap().say(&ctx, "heyyyyyyyyyyyyyyy").await.unwrap();
    println!("what");
}

#[async_traiti]
impl Call {
    async fn new(channel_id: ChannelId) -> Self {
        let id: u64;
        {
            id = rand::random::<u64>();
        }
        println!("{}",id);
        
        connection_id = None;
        Self {
            channel_id
            id,
            connection_id,

        }
    }
}


