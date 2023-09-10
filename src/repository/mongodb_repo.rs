//to handle the environment variable
use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error},
    results::{InsertOneResult},
    Client, Collection,
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var(MONGOURI) {
            Ok(v) => v.to_string(),
            Err(_) => format!(Error loading env variable),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col:Collection<User> = db.Collection("User");
        MongoRepo { col }
    }

}
