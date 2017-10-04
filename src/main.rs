#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_json;

#[get("/users/sorin", format = "application/activity+json")]
fn user() -> rocket_contrib::Json<serde_json::Value> {
    rocket_contrib::Json(json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
        ],
        "id": "http://localhost:8000/users/sorin",
        "type": "Person",
        "following": "http://localhost:8000/users/sorin/following",
        "followers": "http://localhost:8000/users/sorin/followers",
        "inbox": "http://localhost:8000/users/sorin/inbox",
        "outbox": "http://localhost:8000/users/sorin/outbox",
        "preferredUsername": "sorin",
        "name": "Sorin Davidoi",
        "summary": "Cobalt user.",
        "url": "http://localhost:8000/@sorin",
    }))
}

fn main() {
    rocket::ignite().mount("/", routes![user]).launch();
}
