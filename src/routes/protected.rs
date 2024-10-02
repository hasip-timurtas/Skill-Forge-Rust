use crate::services::auth::Claims;

#[get("/protected")]
pub fn protected_route(_claims: Claims) -> &'static str {
    "This is a protected route, accessible only with a valid token."
}