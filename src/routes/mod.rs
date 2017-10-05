extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

pub fn create() -> Vec<rocket::Route> {
    routes!(user, user_following, user_followers, user_outbox)
}

#[get("/users/<username>", format = "application/activity+json")]
fn user(username: String) -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(database::users::fetch(username).as_activity_pub())
}

#[get("/users/<username>/following", format = "application/activity+json")]
fn user_following(username: String) -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(database::users::fetch_following(username).as_activity_pub())
}

#[get("/users/<username>/followers", format = "application/activity+json")]
fn user_followers(username: String) -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(database::users::fetch_followers(username).as_activity_pub())
}

#[get("/users/<username>/outbox", format = "application/activity+json")]
fn user_outbox(username: String) -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(database::users::fetch_outbox(username).as_activity_pub())
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for database::Connection {
    type Error = ();

    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<database::Connection, ()> {
        let pool = request.guard::<rocket::State<database::Pool>>()?;

        match pool.get() {
            Ok(connection) => rocket::Outcome::Success(database::Connection(connection)),
            Err(_) => rocket::Outcome::Failure((rocket::http::Status::ServiceUnavailable, ())),
        }
    }
}
