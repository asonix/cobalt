#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod database;

#[get("/users/sorin", format = "application/activity+json")]
fn user() -> rocket_contrib::Json<serde_json::Value> {
    let user = database::users::fetch();

    rocket_contrib::Json(json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
        ],
        "id": format!("http://localhost:8000/users/{}", user.username),
        "type": "Person",
        "following": format!("http://localhost:8000/users/{}/following", user.username),
        "followers": format!("http://localhost:8000/users/{}/followers", user.username),
        "inbox": format!("http://localhost:8000/users/{}/inbox", user.username),
        "outbox": format!("http://localhost:8000/users/{}/outbox", user.username),
        "preferredUsername": user.username,
        "name": user.name,
        "summary": user.summary,
        "url": format!("http://localhost:8000/@{}", user.username),
    }))
}

fn main() {
    rocket::ignite().mount("/", routes![user]).launch();
}
