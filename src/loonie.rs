use serenity::async_trait;
use serenity::client::bridge::gateway::ShardMessenger;
use serenity::framework::standard::help_commands::Command;
use serenity::gateway;
use serenity::model::prelude::Activity;
use serenity::model::user::CurrentUser;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::builder;

use crate::vrchat::{self, ConnectionConfig};
use crate::vrchat::ApiConnection;

#[group]
#[commands(hi)]
struct General;

#[hook]
async fn before() {
    async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
        println!("Running command '{}' invoked by '{}'", command_name, msg.author.tag());
    
        true
    }
}

#[group]
#[prefixes("vrc")]
#[commands(active)]
struct Vrchat;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

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
            .group(&VRCHAT_GROUP);
    
        
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(&self.discord_token, intents)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client");

            {
                let mut data = client.data.write().await;
    
                data.insert::<ConnectionConfig>(self.vrc_api_connection.config.clone());
            }
    
            if let Err(why) = &mut client.start().await {
                println!("An error occurred while running the client: {:?}", why);
            }
        }
}



#[command]
async fn hi(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Sup, bitch").await?;

    Ok(())
}

#[command]
async fn active(ctx: &Context, msg: &Message) -> CommandResult {
    let lock = ctx.data.read().await;
    let config = lock.get::<ConnectionConfig>().expect("Expected ConnectionConfig in TypeMap").clone();

    let online_users = vrchat::get_online_players(config);
    let mut message: String = String::from("Currently active VRChat users: ");
    message.push_str(&online_users.to_string());

    if let Err(_) = ctx.cache.guild_channel(msg.channel_id).unwrap().send_message(ctx, |m| m.content(message)).await {
        msg.reply(ctx, "Something broke, idk lmao.  Ping Vivid I guess").await;
    }

    Ok(())
}