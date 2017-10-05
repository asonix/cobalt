extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

pub fn create() -> Vec<rocket::Route> {
    routes!(user)
}

#[get("/users/sorin", format = "application/activity+json")]
fn user() -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(database::users::fetch().as_activity_pub())
}
