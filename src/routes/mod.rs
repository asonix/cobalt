extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

pub fn create() -> Vec<rocket::Route> {
    routes!(user, user_followers)
}

#[get("/users/<username>", format = "application/activity+json")]
fn user(username: String) -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(database::users::fetch(username).as_activity_pub())
}

#[get("/users/<username>/followers", format = "application/activity+json")]
fn user_followers(username: String) -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(database::users::fetch_followers(username).as_activity_pub())
}
