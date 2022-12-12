mod loonie;
mod vrchat;
mod commands;

use std::env::var;
use dotenv::dotenv;

use self::vrchat::ApiConnection;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = var("DISCORD_TOKEN").expect("token");
    let vrc_user = var("VRCHAT_USERNAME").expect("vrchat username");
    let vrc_pass = var("VRCHAT_PASSWORD").expect("vrchat password");
    
    let vrc_api_connection = ApiConnection::init(vrc_user, vrc_pass);

    let loona = loonie::Bot::new(token, vrc_api_connection);
    loona.connect().await;
}