extern crate rocket_contrib;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

#[get("/users/<username>/followers", format = "application/activity+json")]
fn handler(username: String) -> Option<rocket_contrib::Json<serde_json::Value>> {
    match database::users::fetch_followers(username) {
        Some(followers) => Some(rocket_contrib::Json(followers.as_activity_pub())),
        None => None,
    }
}
