extern crate serde_json;

pub trait ActivityPub {
    fn as_activity_pub(&self) -> serde_json::Value {
        panic!("Not implemented!");
    }
}
