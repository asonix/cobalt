extern crate serde_json;

use database;

pub trait ActivityPub {
    fn as_activity_pub(&self) -> serde_json::Value {
        panic!("Not implemented!");
    }
}

impl ActivityPub for database::users::User {
    fn as_activity_pub(&self) -> serde_json::Value {
        json!({
            "@context": [
                "https://www.w3.org/ns/activitystreams",
            ],
            "id": format!("http://localhost:8000/users/{}", self.username),
            "type": "Person",
            "following": format!("http://localhost:8000/users/{}/following", self.username),
            "followers": format!("http://localhost:8000/users/{}/followers", self.username),
            "inbox": format!("http://localhost:8000/users/{}/inbox", self.username),
            "outbox": format!("http://localhost:8000/users/{}/outbox", self.username),
            "preferredUsername": self.username,
            "name": self.name,
            "summary": self.summary,
            "url": format!("http://localhost:8000/@{}", self.username),
        })
    }
}
