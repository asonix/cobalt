#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod database;
mod activity_pub;
mod routes;

fn main() {
    rocket::ignite().mount("/", routes::create()).launch();
}
