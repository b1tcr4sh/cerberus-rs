use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::{Message, Reaction};
use serenity::framework::standard::macros::{group, hook};
use serenity::framework::standard::StandardFramework;

use crate::vrchat::ConnectionConfig;
use crate::vrchat::ApiConnection;
use crate::commands::{general::*, vrc::*, util::*};
use crate::reaction_handler::*;

#[group]
#[commands(hi)]
struct General;
#[group]
#[prefixes("vrc")]
#[commands(active)]
struct Vrchat;
#[group]
#[commands(reaction)]
struct Util;

#[hook]
async fn before() {
    async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
        println!("Running command '{0}' invoked by '{1}'", command_name, msg.author.tag());
    
        true
    }
}
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        for guild in ready.guilds {
            if !guild.unavailable {
                if let Some(guild) = ctx.cache.guild(guild.id) {
                    if let Some(channel) = &guild.channel_id_from_name(&ctx.cache, "general") {
                        println!("Greeting server {0}", guild.name);
                        if let Err(why) = channel.say(&ctx.http, "Sup Bitches!").await {
                            eprintln!("{:?}", why);
                        }
                    }
                }
            } else {
                println!("{0} is unavailable", guild.id);
            }
        }
    }
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let partial_member = reaction.member.expect("Couldn't get member from reaction... ?");
        let user = partial_member.user.expect("Couldn't get user from reaction");
        let guild_id = &reaction.guild_id.expect("Couldn't get guild id from cache... ?");

        if (user.id == ctx.cache.current_user_id()) {
            return;
        }

        let lock = ctx.data.read().await;
        let Some(handler) = lock.get::<ReactionHandler>() else {
            eprint!("Failed to fetch ReactionHander from data lock");
            return;
        };

        // Get Custom from reaction.emoji for EmojiId

        
        // handler.get(&reaction.message_id, emoji);
        let Some(guild_id) = reaction.guild_id else {
            eprintln!("Could not get guild id from reaction... ?");
            return;
        };
        let Some(member) = &ctx.cache.member(guild_id, user.id) else {
            eprintln!("Could not get member from cache... ?");
            return;
        };
        
        // Get reaction listener
        if let Some(listener) = handler.get(&reaction.message_id, &reaction.emoji) {
            let mut member = ctx.http.get_member(guild_id.0, user.id.0).await.expect("Couldn't get member from guild.. ?");

            member.add_role(&ctx, listener.role_id).await.expect("Couldn't add role to user... ?");

        } else {
            return;
        }


    }
}

pub struct Bot {
    discord_token: String,
    vrc_api_connection: ApiConnection
}

impl Bot {
    pub fn new(token: String, connection: ApiConnection) -> Bot {
        Bot { discord_token: token, vrc_api_connection: connection }
    }

    pub async fn connect(&self) {
        let framework = StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .group(&GENERAL_GROUP)
            .group(&VRCHAT_GROUP)
            .group(&UTIL_GROUP);
    
        
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(&self.discord_token, intents)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client");

            {
                let mut data = client.data.write().await;
    
                data.insert::<ConnectionConfig>(self.vrc_api_connection.config.clone());
                data.insert::<ReactionHandler>(ReactionHandler::new());
            }
    
            if let Err(why) = &mut client.start().await {
                println!("An error occurred while running the client: {:?}", why);
            }
        }
}