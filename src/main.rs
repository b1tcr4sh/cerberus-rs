mod loonie;
mod vrchat;

use std::env::var;

#[tokio::main]
async fn main() {
    let token = var("DISCORD_TOKEN").expect("token");
    let vrc_user = var("VRCHAT_USERNAME").expect("vrchat username");
    let vrc_pass = var("VRCHAT_PASSWORD").expect("vrchat password");
    
    let vrc_api_connection = vrchat::ApiConnection::init(vrc_user, vrc_pass);

    let loona = loonie::Bot::new(token, vrc_api_connection);
    loona.connect().await;
}