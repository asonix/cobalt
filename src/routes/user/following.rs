extern crate rocket_contrib;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

#[get("/users/<username>/following", format = "application/activity+json")]
fn handler(username: String) -> Option<rocket_contrib::Json<serde_json::Value>> {
    match database::users::fetch_following(username) {
        Some(following) => Some(rocket_contrib::Json(following.as_activity_pub())),
        None => None,
    }
}
