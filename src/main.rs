#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod database;
mod activity_pub;
mod routes;

fn main() {
    rocket::ignite()
        .manage(database::create_connection_pool())
        .mount("/api", routes::create())
        .launch();
}
