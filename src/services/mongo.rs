use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use mongodb::bson::doc;

/// Initializes the MongoDB client and checks the connection.
///
/// Loads environment variables from `.env`, parses the MongoDB URI,
/// creates and configures the MongoDB client, and pings the MongoDB server
/// to ensure the connection is successful.
///
/// @returns {Client} A configured MongoDB client.
pub async fn init_mongo() -> Client {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let client_options = ClientOptions::parse(&mongodb_uri)
        .await
        .expect("Failed to parse MongoDB URI");
    let client = Client::with_options(client_options).expect("Failed to initialize MongoDB client");
    client.database("user_db")
        .run_command(doc! {"ping": 1}, None)
        .await
        .expect("Failed to ping MongoDB");
    println!("MongoDB connection successful");
    client
}
