use dotenv;
use std::env;

pub struct Config {
    pub mongo_url: String,
    pub port: String,
    pub database: String,
    pub collection: String,
}

pub fn new() -> Config {
    dotenv::dotenv().ok();
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let database = env::var("MONGO_DB").expect("MONGO_DB must be set");
    let collection = env::var("MONGO_COLLECTION").expect("COLLECTION must be set");

    Config {
        mongo_url,
        port,
        database,
        collection,
    }
}
