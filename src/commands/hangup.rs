use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
};

use crate::callhandler::{get_connection,end_call};

pub async fn hangup(ctx: &Context, msg: &Message) {
    let connection: ChannelId = match get_connection(msg.channel_id).await {
        Some(id) => id,
        None => {
            msg.channel_id.say(ctx, "Not connected to a call").await;
            return;
        },
    };
    

    end_call(msg.channel_id,connection,ctx).await;
}
