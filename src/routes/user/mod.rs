extern crate rocket_contrib;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

pub mod following;
pub mod followers;
pub mod inbox;
pub mod outbox;

#[get("/users/<username>", format = "application/activity+json")]
fn handler(
    username: String,
    conn: database::Connection,
) -> Option<rocket_contrib::Json<serde_json::Value>> {
    match database::users::fetch(username, conn) {
        Some(user) => Some(rocket_contrib::Json(user.as_activity_pub())),
        None => None,
    }
}
