use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;
use user_login_api::{rocket, User};

#[rocket::async_test]
async fn test_register_success() {
    let client = Client::tracked(rocket())
        .await
        .expect("valid rocket instance");
    let response = client
        .post("/register")
        .json(&json!({
            "email": "new_user6@example.com",
            "password": "newpassword",
            "name": "New User",
            "age": 25,
            "eth_address": "0xabcdef1234567890"
        }))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let user: User = response.into_json().await.expect("valid user");
    assert_eq!(user.email, "new_user6@example.com");
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
    let response = client
        .post("/login")
        .json(&json!({"email": "test@example.com", "password": "password123"}))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let user: User = response.into_json().await.expect("valid user");
    assert_eq!(user.email, "test@example.com");
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
