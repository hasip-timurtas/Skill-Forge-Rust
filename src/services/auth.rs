use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, (Status, Self::Error), ()> {
        let auth_headers: Vec<_> = request.headers().get("Authorization").collect();

        if auth_headers.len() != 1 {
            return Outcome::Error((Status::Unauthorized, (Status::Unauthorized, ())));
        }

        let token = auth_headers[0]
            .trim_start_matches("Bearer ")
            .trim()
            .to_string();

        let secret = match std::env::var("JWT_SECRET") {
            Ok(secret) => secret,
            Err(_) => return Outcome::Error((Status::InternalServerError, (Status::Unauthorized, ()))),
        };

        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => Outcome::Success(token_data.claims),
            Err(_) => Outcome::Error((Status::Unauthorized, (Status::Unauthorized, ()))),
        }
    }
}
