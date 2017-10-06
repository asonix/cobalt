extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use activity_pub::ActivityPub;
use database;

pub fn create() -> Vec<rocket::Route> {
    routes!(register, user, user_following, user_followers, user_outbox)
}

#[post("/register", data = "<user>", format = "application/json")]
fn register(user: rocket_contrib::Json<database::users::RegisterUser>, conn: database::Connection) {
    database::users::register(user.0, conn);
}

#[get("/users/<username>", format = "application/activity+json")]
fn user(
    username: String,
    conn: database::Connection,
) -> Option<rocket_contrib::Json<serde_json::Value>> {
    match database::users::fetch(username, conn) {
        Some(user) => Some(rocket_contrib::Json(user.as_activity_pub())),
        None => None,
    }
}

#[get("/users/<username>/following", format = "application/activity+json")]
fn user_following(username: String) -> Option<rocket_contrib::Json<serde_json::Value>> {
    match database::users::fetch_following(username) {
        Some(following) => Some(rocket_contrib::Json(following.as_activity_pub())),
        None => None,
    }
}

#[get("/users/<username>/followers", format = "application/activity+json")]
fn user_followers(username: String) -> Option<rocket_contrib::Json<serde_json::Value>> {
    match database::users::fetch_followers(username) {
        Some(followers) => Some(rocket_contrib::Json(followers.as_activity_pub())),
        None => None,
    }
}

#[get("/users/<username>/outbox", format = "application/activity+json")]
fn user_outbox(username: String) -> Option<rocket_contrib::Json<serde_json::Value>> {
    match database::users::fetch_outbox(username) {
        Some(outbox) => Some(rocket_contrib::Json(outbox.as_activity_pub())),
        None => None,
    }
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
