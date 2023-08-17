use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
};


pub async fn parse_channelid(id: &str) ->  Result<ChannelId, ()> {
    Ok(ChannelId(parse_channel(id).unwrap())) 
}
