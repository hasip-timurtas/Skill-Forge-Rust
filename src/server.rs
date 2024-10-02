use rocket::{fairing::AdHoc, Rocket};

/// Initializes and configures the Rocket instance.
///
/// - Attaches MongoDB initialization as a fairing.
/// - Mounts routes for index, login, and register.
///
/// @returns {rocket::Rocket<rocket::Build>} A configured Rocket instance.
pub fn rocket() -> Rocket<rocket::Build> {
    rocket::build()
        .attach(AdHoc::on_ignite("MongoDB Init", |rocket| async {
            let client = crate::services::mongo::init_mongo().await;
            rocket.manage(client)
        }))
        .mount("/", routes![crate::routes::index::index, crate::routes::auth::login, crate::routes::auth::register])
}
