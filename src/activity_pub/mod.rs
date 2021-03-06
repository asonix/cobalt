extern crate serde_json;

use database;

#[derive(Debug, Deserialize)]
pub enum InboxTypes {
    Follow,
}

#[derive(Deserialize)]
pub struct Inbox {
    pub id: String,
    pub actor: String,
    #[serde(rename = "type")]
    pub _type: InboxTypes,
    pub object: String,
}

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
            "id": format!("http://localhost:8000/api/users/{}", self.username),
            "type": "Person",
            "following": format!("http://localhost:8000/api/users/{}/following", self.username),
            "followers": format!("http://localhost:8000/api/users/{}/followers", self.username),
            "inbox": format!("http://localhost:8000/api/users/{}/inbox", self.username),
            "outbox": format!("http://localhost:8000/api/users/{}/outbox", self.username),
            "preferredUsername": self.username,
            "name": self.name,
            "summary": self.summary,
            "url": format!("http://localhost:8000/@{}", self.username),
        })
    }
}

impl ActivityPub for database::users::Following {
    fn as_activity_pub(&self) -> serde_json::Value {
        json!({
            "@context": [
                "https://www.w3.org/ns/activitystreams",
            ],
            "id": format!("http://localhost:8000/api/users/{}/following", self.username),
            "type": "OrderedCollection",
            "totalItems": self.following.len(),
            "orderedItems": self.following
                .iter()
                .map(|follower| format!("http://localhost:8000/api/users/{}", follower.username))
                .collect::<Vec<String>>(),
        })
    }
}

impl ActivityPub for database::users::Followers {
    fn as_activity_pub(&self) -> serde_json::Value {
        json!({
            "@context": [
                "https://www.w3.org/ns/activitystreams",
            ],
            "id": format!("http://localhost:8000/api/users/{}/followers", self.username),
            "type": "OrderedCollection",
            "totalItems": self.followers.len(),
            "orderedItems": self.followers
                .iter()
                .map(|follower| format!("http://localhost:8000/api/users/{}", follower.username))
                .collect::<Vec<String>>(),
        })
    }
}

impl ActivityPub for database::users::Outbox {
    fn as_activity_pub(&self) -> serde_json::Value {
        json!({
            "@context": [
                "https://www.w3.org/ns/activitystreams",
            ],
            "id": format!("http://localhost:8000/api/users/{}/outbox", self.username),
            "type": "OrderedCollection",
            "totalItems": 0,
            "orderedItems": [],
        })
    }
}
