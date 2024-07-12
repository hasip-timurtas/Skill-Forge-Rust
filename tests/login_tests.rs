use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;
use user_login_api::{rocket, User};

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
