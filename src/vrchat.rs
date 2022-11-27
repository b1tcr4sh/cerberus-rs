use vrchatapi::apis;
use vrchatapi::models::*;

pub struct ApiConnection {}

impl ApiConnection {
    pub fn init(vrchat_username: String, vrchat_password: String) -> ApiConnection {
        let mut config = apis::configuration::Configuration::default();
        config.basic_auth = Some((vrchat_username, Some(vrchat_password)));    

        ApiConnection {  }
    }
}