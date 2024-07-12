#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = user_login_api::rocket().launch().await?;
    Ok(())
}
