use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::*;
use sqlx::Connection;
use serenity::futures::TryStreamExt;

use crate::commands::call::Call;


pub async fn insert_call() -> Result<()> {
    let mut conn = SqliteConnection::connect(DB_URL).await?;

    let (gid, uid) = (guildid.to_string(), userid.to_string());
    let cursetypedb: i64 = match cursetype {
        CurseType::DumbName => 0,
        CurseType::NoThe => 1,
        _ => {panic!("Supplied curse type is not a valid curse type");}
    };
    sqlx::query!("INSERT INTO curses (guild_id, user_id, time_uncurse, curse_type) VALUES ($1, $2, $3, $4)", gid, uid, ts_uncurse, cursetypedb)
        .execute(&mut conn)
        .await?;

    Ok(())
}
