#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use rocket::fairing::AdHoc;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: String,
    password: String,
    name: String,
    age: i32,
    eth_address: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login", data = "<login_request>")]
async fn login(login_request: Json<LoginRequest>, client: &State<Client>) -> Option<Json<User>> {
    let users_collection = client.database("user_db").collection::<User>("users");

    let filter = doc! {
        "email": &login_request.email,
        "password": &login_request.password,
    };

    match users_collection.find_one(filter, None).await {
        Ok(Some(user)) => Some(Json(user)),
        Ok(None) => None,
        Err(_) => None,
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn init_mongo() -> Client {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let client_options = ClientOptions::parse(&mongodb_uri).await.unwrap();
    Client::with_options(client_options).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("MongoDB Init", |rocket| async {
            let client = init_mongo().await;
            rocket.manage(client)
        }))
        .mount("/", routes![index, login])
}
