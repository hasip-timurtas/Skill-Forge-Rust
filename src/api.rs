use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use rocket::fairing::AdHoc;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub name: String,
    pub age: i32,
    pub eth_address: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<login_request>")]
pub async fn login(
    login_request: Json<LoginRequest>,
    client: &State<Client>,
) -> Option<Json<User>> {
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
pub fn index() -> &'static str {
    "Hello, world!"
}

pub async fn init_mongo() -> Client {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let client_options = ClientOptions::parse(&mongodb_uri).await.unwrap();
    Client::with_options(client_options).unwrap()
}

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(AdHoc::on_ignite("MongoDB Init", |rocket| async {
            let client = init_mongo().await;
            rocket.manage(client)
        }))
        .mount("/", routes![index, login])
}
