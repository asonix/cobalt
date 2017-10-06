extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate std;

mod schema;

use self::r2d2_diesel::ConnectionManager;

pub const DATABASE_FILE: &'static str = env!("DATABASE_URL");

pub type Pool = r2d2::Pool<ConnectionManager<diesel::PgConnection>>;

pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<diesel::PgConnection>>);

impl std::ops::Deref for Connection {
    type Target = diesel::PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_connection_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<diesel::PgConnection>::new(DATABASE_FILE);

    r2d2::Pool::new(config, manager).unwrap()
}

pub mod users {
    use diesel;
    use diesel::ExecuteDsl;
    use database::Connection;
    use database::schema::users;

    pub struct User {
        pub username: String,
        pub name: String,
        pub summary: String,
    }

    #[derive(Insertable, Serialize, Deserialize)]
    #[table_name = "users"]
    pub struct RegisterUser {
        pub username: String,
        pub password: String,
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

    pub fn register(user: RegisterUser, conn: Connection) {
        diesel::insert(&user)
            .into(users::table)
            .execute(&*conn)
            .unwrap();
    }

    pub fn fetch(_username: String) -> Option<User> {
        Some(User {
            username: "sorin".to_string(),
            name: "Sorin Davidoi".to_string(),
            summary: "Cobalt user".to_string(),
        })
    }

    pub fn fetch_following(username: String) -> Option<Following> {
        Some(Following {
            username,
            following: vec![
                User {
                    username: "ghost".to_string(),
                    name: "Ghost".to_string(),
                    summary: "Cobalt ghost user.".to_string(),
                },
            ],
        })
    }

    pub fn fetch_followers(username: String) -> Option<Followers> {
        Some(Followers {
            username,
            followers: vec![
                User {
                    username: "ghost".to_string(),
                    name: "Ghost".to_string(),
                    summary: "Cobalt ghost user.".to_string(),
                },
            ],
        })
    }

    pub fn fetch_outbox(username: String) -> Option<Outbox> {
        Some(Outbox {
            username,
            outbox: vec![Post {}],
        })
    }
}
