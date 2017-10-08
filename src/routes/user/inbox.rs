extern crate rocket;
extern crate serde_json;

use activity_pub::{Inbox, InboxTypes};

#[post("/users/<_username>/inbox", data = "<inbox>", format = "application/activity+json")]
pub fn handler(_username: String, inbox: Inbox) {
    match inbox._type {
        InboxTypes::Follow => {
            panic!("Not implemented");
        }
    }
}

impl rocket::data::FromData for Inbox {
    type Error = serde_json::Error;

    fn from_data(
        _request: &rocket::request::Request,
        data: rocket::Data,
    ) -> rocket::data::Outcome<Self, serde_json::Error> {
        match serde_json::from_reader(data.open()) {
            Ok(inbox) => rocket::Outcome::Success(inbox),
            Err(e) => rocket::Outcome::Failure((rocket::http::Status::BadRequest, e)),
        }
    }
}
