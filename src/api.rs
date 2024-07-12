use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use rocket::fairing::AdHoc;
use rocket::response::status::{Conflict, Unauthorized};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub age: i32,
    pub eth_address: String,
}

#[post("/login", data = "<login_request>")]
pub async fn login(
    login_request: Json<LoginRequest>,
    client: &State<Client>,
) -> Result<Json<User>, Unauthorized<String>> {
    let users_collection = client.database("SkillForge").collection::<User>("users");

    let filter = doc! {
        "email": &login_request.email,
        "password": &login_request.password,
    };

    match users_collection.find_one(filter, None).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Unauthorized("Invalid email or password".to_string())),
        Err(_) => Err(Unauthorized("Database error".to_string())),
    }
}

#[post("/register", data = "<register_request>")]
pub async fn register(
    register_request: Json<RegisterRequest>,
    client: &State<Client>,
) -> Result<Json<User>, Conflict<&'static str>> {
    let users_collection = client.database("SkillForge").collection::<User>("users");

    let new_user = User {
        email: register_request.email.clone(),
        password: register_request.password.clone(),
        name: register_request.name.clone(),
        age: register_request.age,
        eth_address: register_request.eth_address.clone(),
    };

    let filter = doc! {
        "email": &register_request.email,
    };

    match users_collection.find_one(filter, None).await {
        Ok(Some(_)) => Err(Conflict("User already exists")),
        Ok(None) => {
            users_collection
                .insert_one(new_user.clone(), None)
                .await
                .expect("Failed to insert new user");
            Ok(Json(new_user))
        }
        Err(_) => Err(Conflict("Failed to query database")),
    }
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

pub async fn init_mongo() -> Client {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let client_options = ClientOptions::parse(&mongodb_uri)
        .await
        .expect("Failed to parse MongoDB URI");
    let client = Client::with_options(client_options).expect("Failed to initialize MongoDB client");

    // Check MongoDB connection
    client
        .database("user_db")
        .run_command(doc! {"ping": 1}, None)
        .await
        .expect("Failed to ping MongoDB");
    println!("MongoDB connection successful");

    client
}

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(AdHoc::on_ignite("MongoDB Init", |rocket| async {
            let client = init_mongo().await;
            rocket.manage(client)
        }))
        .mount("/", routes![index, login, register])
}
