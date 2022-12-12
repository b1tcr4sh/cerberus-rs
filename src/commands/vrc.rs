use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::prelude::Context;

use crate::vrchat::{self, ConnectionConfig};

#[command]
async fn active(ctx: &Context, msg: &Message) -> CommandResult {
    let lock = ctx.data.read().await;
    let config = lock.get::<ConnectionConfig>().expect("Expected ConnectionConfig in TypeMap").clone();

    let online_users = vrchat::get_online_players(config);
    let mut message: String = String::from("Currently active VRChat users: ");
    message.push_str(&online_users.to_string());

    if let Err(_) = ctx.cache.guild_channel(msg.channel_id).unwrap().send_message(ctx, |m| m.content(message)).await {
        msg.reply(ctx, "Something broke, idk lmao.  Ping Vivid I guess").await.expect("Something brokey");
    }

    Ok(())
}