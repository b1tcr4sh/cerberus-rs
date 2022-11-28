use vrchatapi::apis;
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::models::*;

pub struct ApiConnection {
    config: Configuration
}

impl ApiConnection {
    pub fn init(vrchat_username: String, vrchat_password: String) -> ApiConnection {
        let mut config = apis::configuration::Configuration::default();
        config.basic_auth = Some((vrchat_username, Some(vrchat_password)));    

        ApiConnection { config: config }
    }

    pub fn GetOnlinePlayers(&self) -> u64 {
        self.GetOnlinePlayers()
    }
}