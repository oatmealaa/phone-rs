use serenity::{
    prelude::*,
    model::prelude::*,
    utils::*,
    model::channel::Message,
};

const INFO_PREFIX: &str = "[INFO]:";

pub enum Info {
    Ready,
    CallStarted(u64, u64),
    CallPending(u64),
    Message(String ,u64, u64, String),
    CallEnded(u64, u64),
}

pub async fn log(token: Info) {
    match token {
        Info::Ready => println!("{}Phone-rs is ready", INFO_PREFIX),
        Info::CallStarted(cid1,cid2) => println!("{}Call Started {} <-> {}",INFO_PREFIX, cid1, cid2),
        Info::CallPending(cid) => println!("{}{} Entered pending que", INFO_PREFIX, cid),
        Info::Message(name,cid,cid2,msg) => println!("{} Message {} -> {} {}: {} ", INFO_PREFIX, cid, cid2, name, msg),
        Info::CallEnded(cid,cid2) => println!("{}Call ended {} <-> {}", INFO_PREFIX, cid ,cid2),
    }
}
