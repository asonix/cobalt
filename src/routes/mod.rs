extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

pub fn create() -> Vec<rocket::Route> {
    routes!(user)
}

#[get("/users/<username>", format = "application/activity+json")]
fn user(username: String) -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(database::users::fetch(username).as_activity_pub())
}
