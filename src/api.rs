use dotenv::dotenv;
use md5::{Digest, Md5};
use mongodb::{bson::doc, options::ClientOptions, Client};
use rand::thread_rng; // ThreadRng kullanımı için ekleyin
use rand::Rng;
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
    pub salt: String,
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
) -> Result<Json<User>, Unauthorized<&'static str>> {
    let users_collection = client.database("SkillForge").collection::<User>("users");

    let filter = doc! {
        "email": &login_request.email,
    };

    match users_collection.find_one(filter, None).await {
        Ok(Some(user)) => {
            let salt_str = &user.salt;
            let salted_password = format!("{}{}", login_request.password, salt_str);
            let mut hasher = Md5::new();
            hasher.update(salted_password);
            let hashed_password = format!("{:x}", hasher.finalize());

            if hashed_password == user.password {
                Ok(Json(user))
            } else {
                Err(Unauthorized("Invalid email or password"))
            }
        }
        Ok(None) => Err(Unauthorized("Invalid email or password")),
        Err(_) => Err(Unauthorized("Database error")),
    }
}

#[post("/register", data = "<register_request>")]
pub async fn register(
    register_request: Json<RegisterRequest>,
    client: &State<Client>,
) -> Result<Json<User>, Conflict<&'static str>> {
    let users_collection = client.database("SkillForge").collection::<User>("users");

    // Generate a random salt
    let salt: u32 = thread_rng().gen();
    let salt_str = salt.to_string();

    // Hash the password with the salt
    let salted_password = format!("{}{}", register_request.password, salt_str);
    let mut hasher = Md5::new();
    hasher.update(salted_password);
    let hashed_password = format!("{:x}", hasher.finalize());

    let new_user = User {
        email: register_request.email.clone(),
        password: hashed_password,
        name: register_request.name.clone(),
        age: register_request.age,
        eth_address: register_request.eth_address.clone(),
        salt: salt_str, // Save the salt
    };

    let filter = doc! {
        "email": &register_request.email,
    };

    match users_collection.find_one(filter, None).await {
        Ok(Some(_)) => Err(Conflict("User already exists")),
        Ok(None) => {
            // Store the user with the hashed password and salt
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
