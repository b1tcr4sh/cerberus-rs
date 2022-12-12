use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::prelude::Context;

#[command]
async fn hi(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Sup, bitch").await?;

    Ok(())
}