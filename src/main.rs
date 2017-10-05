#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod database;
mod activity_pub;

#[get("/users/sorin", format = "application/activity+json")]
fn user() -> rocket_contrib::Json<serde_json::Value> {
    use activity_pub::ActivityPub;
    rocket_contrib::Json(database::users::fetch().as_activity_pub())
}

fn main() {
    rocket::ignite().mount("/", routes![user]).launch();
}
