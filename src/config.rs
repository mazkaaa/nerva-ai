use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Config {
            api_url: env::var("API_URL").expect("API_URL must be set"),
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
        }
    }
}
