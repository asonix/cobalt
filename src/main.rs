#![recursion_limit="128"]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate futures;
extern crate hyper;
extern crate rocket;
extern crate rocket_cors;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;

mod database;
mod activity_pub;
mod routes;

fn main() {
    rocket::ignite()
        .manage(database::create_connection_pool())
        .attach(routes::create_cors_options())
        .mount("/api", routes::create())
        .launch();
}
