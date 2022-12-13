use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, Args, CommandError};
use serenity::prelude::Context;
use serenity::model::id::{MessageId, RoleId, EmojiId};

use crate::reaction_handler::ReactionHandler;

#[command]
#[num_args(3)]
#[usage("reaction [Message ID]<string> [Role ID]<string> [Emoji]<emoji>")]
async fn reaction(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut args_vec: Vec<String> = Vec::new(); 
    
    for arg in args.iter::<String>() {
        if let Ok(param) = arg {
            args_vec.push(param);
        } else {
            msg.reply(ctx, "Bitch something's wrong with the command arguments, try again?").await.unwrap();
        }
    }

    let Some(message_id) = args_vec.get(0) else {
        msg.reply(ctx, "Bitch something's wrong with the command arguments, try again?").await.unwrap();
        return CommandResult::Err(CommandError::from("Command args were None"));
    };
    let Some(role_id) = args_vec.get(1) else {
        msg.reply(ctx, "Bitch something's wrong with the command arguments, try again?").await.unwrap();
        return CommandResult::Err(CommandError::from("Command args were None"));
    };
    let Some(emoji) = args_vec.get(2) else {
        msg.reply(ctx, "Bitch something's wrong with the command arguments, try again?").await.unwrap();
        return CommandResult::Err(CommandError::from("Command args were None"));
    };

    let lock = ctx.data.read().await;
    let Some(reactions) = lock.get::<ReactionHandler>() else {
        return CommandResult::Err(CommandError::from("Failed to fetch ReactionHander from data lock"));
    };

    // reactions.register(MessageId::from(message_id), RoleId::from(role_id), );
    // Parse emoji from message string

    println!("{0}, {1}, {2}", message_id, role_id, emoji);

    Ok(())
}