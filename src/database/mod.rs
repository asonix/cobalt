pub mod users {
    pub struct User {
        pub username: String,
        pub name: String,
        pub summary: String,
    }

    pub struct Following {
        pub username: String,
        pub following: Vec<User>,
    }

    pub struct Followers {
        pub username: String,
        pub followers: Vec<User>,
    }

    pub struct Post {}

    pub struct Outbox {
        pub username: String,
        pub outbox: Vec<Post>,
    }

    pub fn fetch(_username: String) -> User {
        User {
            username: "sorin".to_string(),
            name: "Sorin Davidoi".to_string(),
            summary: "Cobalt user".to_string(),
        }
    }

    pub fn fetch_following(username: String) -> Following {
        Following {
            username,
            following: vec![
                User {
                    username: "ghost".to_string(),
                    name: "Ghost".to_string(),
                    summary: "Cobalt ghost user.".to_string(),
                },
            ],
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

    pub fn fetch_outbox(username: String) -> Outbox {
        Outbox {
            username,
            outbox: vec![Post {}],
        }
    }
}
