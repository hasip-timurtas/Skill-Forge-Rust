/// The index route handler.
///
/// Returns a simple greeting message.
///
/// @returns {&'static str} A greeting message.
///
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
