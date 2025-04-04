mod db;
use std::env;

fn main() {
    println!("Hello, world!");
}

fn get_api_key() -> String {
    dotenv::dotenv().ok();
    env::var("API_KEY").expect("API key not set in .env!")
}
