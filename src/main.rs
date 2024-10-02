use skill_forge::server::rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket().launch().await?;
    Ok(())
}