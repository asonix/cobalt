pub mod users {
    pub struct User {
        pub username: String,
        pub name: String,
        pub summary: String,
    }

    pub struct Followers {
        pub username: String,
        pub followers: Vec<User>,
    }

    pub fn fetch(_username: String) -> User {
        User {
            username: "sorin".to_string(),
            name: "Sorin Davidoi".to_string(),
            summary: "Cobalt user".to_string(),
        }
    }

    pub fn fetch_followers(username: String) -> Followers {
        Followers {
            username,
            followers: vec![
                User {
                    username: "ghost".to_string(),
                    name: "Ghost".to_string(),
                    summary: "Cobalt ghost user.".to_string(),
                },
            ],
        }
    }
}
