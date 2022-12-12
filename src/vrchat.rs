use vrchatapi::apis;
use vrchatapi::apis::configuration::Configuration;
use serenity::prelude::TypeMapKey;

pub struct ApiConnection {
    pub config: Configuration
}

impl ApiConnection {
    pub fn init(vrchat_username: String, vrchat_password: String) -> ApiConnection {
        let mut config = apis::configuration::Configuration::default();
        config.basic_auth = Some((vrchat_username, Some(vrchat_password)));    

        ApiConnection { config: config }
    }
}

pub struct ConnectionConfig { }

impl TypeMapKey for ConnectionConfig {
    type Value = Configuration;
}

pub fn get_online_players(config: Configuration) -> i32 {
    let result = apis::system_api::get_current_online_users(&config);

    match result {
        Err(_) => return -1,
        Ok(online_users) => return online_users
    }
}