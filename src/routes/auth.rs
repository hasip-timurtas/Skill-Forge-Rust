use md5::{Digest, Md5};
use mongodb::{bson::doc, Client};
use rand::thread_rng;
use rand::Rng;
use rocket::serde::{json::Json};
use rocket::State;
use rocket::response::status::{Conflict, Unauthorized};
use crate::models::user::{User, LoginRequest, RegisterRequest};

/// Handles user login.
///
/// Verifies the user's email and password. If the credentials are correct,
/// returns the user data. Otherwise, returns an unauthorized error.
///
/// @param {Json<LoginRequest>} login_request - The login request data.
/// @param {State<Client>} client - The MongoDB client state.
/// @returns {Result<Json<User>, Unauthorized<&'static str>>} The user data or an unauthorized error.
#[post("/login", data = "<login_request>")]
pub async fn login(
    login_request: Json<LoginRequest>,
    client: &State<Client>,
) -> Result<Json<User>, Unauthorized<&'static str>> {
    let users_collection = client.database("SkillForge").collection::<User>("users");
    let filter = doc! { "email": &login_request.email };
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

/// Handles user registration.
///
/// Generates a random salt, hashes the password with the salt,
/// and stores the new user in the MongoDB collection. Returns
/// a conflict error if the user already exists.
///
/// @param {Json<RegisterRequest>} register_request - The registration request data.
/// @param {State<Client>} client - The MongoDB client state.
/// @returns {Result<Json<User>, Conflict<&'static str>>} The newly created user or a conflict error.
#[post("/register", data = "<register_request>")]
pub async fn register(
    register_request: Json<RegisterRequest>,
    client: &State<Client>,
) -> Result<Json<User>, Conflict<&'static str>> {
    let users_collection = client.database("SkillForge").collection::<User>("users");
    let salt: u32 = thread_rng().gen();
    let salt_str = salt.to_string();
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
        salt: salt_str,
    };
    let filter = doc! { "email": &register_request.email };
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
