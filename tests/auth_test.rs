use rand::Rng;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;
use skill_forge::server::rocket;
use skill_forge::models::user::User;

#[rocket::async_test]
async fn test_register_success() {
    let client = Client::tracked(rocket())
        .await
        .expect("valid rocket instance");

    // Generate a random email address
    let random_number: u32 = rand::thread_rng().gen_range(1000..9999);
    let email = format!("new_user{}@example.com", random_number);

    let response = client
        .post("/register")
        .json(&json!({
            "email": email,
            "password": "newpassword",
            "name": "New User",
            "age": 25,
            "eth_address": "0xabcdef1234567890"
        }))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let user: User = response.into_json().await.expect("valid user");
    assert_eq!(user.email, email);
}
#[rocket::async_test]
async fn test_register_existing_user() {
    let client = Client::tracked(rocket())
        .await
        .expect("valid rocket instance");
    let response = client
        .post("/register")
        .json(&json!({
            "email": "test@example.com",
            "password": "password123",
            "name": "Test User",
            "age": 30,
            "eth_address": "0x1234567890abcdef"
        }))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Conflict);
}

#[rocket::async_test]
async fn test_login_success() {
    let client = Client::tracked(rocket())
        .await
        .expect("valid rocket instance");

    // Generate a random email address
    let random_number: u32 = rand::thread_rng().gen_range(1000..9999);
    let email = format!("new_user{}@example.com", random_number);

    // First, register the user
    let register_response = client
        .post("/register")
        .json(&json!({
            "email": &email,
            "password": "password123",
            "name": "Test User",
            "age": 30,
            "eth_address": "0x1234567890abcdef"
        }))
        .dispatch()
        .await;

    assert_eq!(register_response.status(), Status::Ok);
    let user: User = register_response.into_json().await.expect("valid user");
    assert_eq!(user.email, email);

    // Then, login
    let login_response = client
        .post("/login")
        .json(&json!({
            "email": &email,
            "password": "password123"
        }))
        .dispatch()
        .await;

    assert_eq!(login_response.status(), Status::Ok);
    let login_data: (User, String) = login_response.into_json().await.expect("valid user and token");

    // Assert the user data and ensure the JWT token is returned
    assert_eq!(login_data.0.email, email);
    assert!(!login_data.1.is_empty(), "JWT token should not be empty");
}

#[rocket::async_test]
async fn test_login_failure() {
    let client = Client::tracked(rocket())
        .await
        .expect("valid rocket instance");
    let response = client
        .post("/login")
        .json(&json!({"email": "wrong@example.com", "password": "wrongpassword"}))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);
}
