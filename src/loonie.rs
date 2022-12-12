use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{group, hook};
use serenity::framework::standard::StandardFramework;

use crate::vrchat::ConnectionConfig;
use crate::vrchat::ApiConnection;
use crate::commands::{general::*, vrc::*};

#[group]
#[commands(hi)]
struct General;

#[hook]
async fn before() {
    async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
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