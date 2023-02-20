use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, Args};
use serenity::prelude::Context;
use std::time;
use serenity::json::prelude::from_str;

use crate::reaction_handler::{ReactionHandler};

#[command("reaction-role")]
#[num_args(3)]
#[usage("reaction [Message ID]<string> [Role ID]<string>")]
async fn reaction(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut args_vec: Vec<String> = Vec::new(); 
    
    let Some(message_id) = args.current() else {
        msg.reply(ctx, "Missing message ID ~~bitch~~");
        return Ok(());
    };
    let Some(role_id) = args.advance().current() else {
        msg.reply(ctx, "Missing role ID ~~bitch~~");
        return Ok(());
    };


    let lock = ctx.data.read().await;
    let reactions_handler = lock.get::<ReactionHandler>().expect("Failed to get ReactionHandler... ?");
    let guild_id = msg.guild_id.expect("Failed to get guild id from message");
    let roles = ctx.cache.guild_roles(guild_id).expect("Failed to get roles from cache");

    let lock_guard = ctx.data.read().await;
    let reaction_handler = lock_guard.get::<ReactionHandler>().expect("Reaction manager from r/w lock");

    let reaction_request = msg.reply(ctx, "React to this message with the emoji you wish to use").await.unwrap();
    reaction_handler.active_creation_listener = Some(&reaction_request.id);

    let timeout = 0;
    while reaction_handler.paylod() == None {
        if timeout > 10 {
            reaction_request.edit(ctx, |m| m.content("You took too long, sorry (10 secs timeout)"));

            reaction_handler.active_creation_listener = None;
            return Ok(());
        }

        std::thread::sleep(time::Duration::from_secs(1));
        timeout += 1;
    }

    let Some(reaction) = reaction_handler.paylod() else {
        msg.reply(ctx, "Something went wrong, I don't have a fucking clue.");
        return Ok(());
    };


    let Ok(message_id_u64) = from_str::<u64>(message_id) else {
        msg.reply(ctx, "Your first arg is wrong bitch");
        return Ok(());
    };
    let Ok(role_id_u64) = from_str::<u64>(role_id) else {
        msg.reply(ctx, "Your second arg is wrong bitch");
        return Ok(());
    };

    &reactions_handler.register(message_id_u64, role_id_u64, reaction.to_owned());


    Ok(())
}