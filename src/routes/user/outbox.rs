extern crate rocket_contrib;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

#[get("/users/<username>/outbox", format = "application/activity+json")]
fn handler(username: String) -> Option<rocket_contrib::Json<serde_json::Value>> {
    match database::users::fetch_outbox(username) {
        Some(outbox) => Some(rocket_contrib::Json(outbox.as_activity_pub())),
        None => None,
    }
}
