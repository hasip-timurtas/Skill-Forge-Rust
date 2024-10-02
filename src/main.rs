#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = skill_forge::rocket().launch().await?;
    Ok(())
}
