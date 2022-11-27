use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

use crate::vrchat::ApiConnection;

#[group]
#[commands(hi)]
struct General;

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
            .group(&GENERAL_GROUP);
    
        
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
        let mut client = Client::builder(&self.discord_token, intents)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client");
    
            if let Err(why) = client.start().await {
                println!("An error occurred while running the client: {:?}", why);
            }
    }
}



#[command]
async fn hi(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Fuck off").await?;

    Ok(())
}