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
    //
    pub async fn init() -> Self {
        dotenv().ok();
        //load environment variable
        let uri = match env::var(MONGOURI) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");     //create connection to the database
        let col:Collection<User> = db.Collection("User");       //
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .await
            .expect("Error creating user");

        Ok(user)
    }

}
