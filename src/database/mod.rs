pub mod users {
    pub struct User {
        pub username: String,
        pub name: String,
        pub summary: String,
    }

    pub fn fetch() -> User {
        User {
            username: "sorin".to_string(),
            name: "Sorin Davidoi".to_string(),
            summary: "Cobalt user".to_string(),
        }
    }
}
