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
    use diesel::{ExecuteDsl, ExpressionMethods, FindDsl, FilterDsl, FirstDsl};
    use database::Connection;
    use database::schema::users;

    #[derive(Queryable, Serialize)]
    pub struct User {
        #[serde(skip_serializing)]
        pub id: i32,
        pub username: String,
        #[serde(skip_serializing)]
        pub password: String,
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

    #[derive(Deserialize)]
    pub struct LoginUser {
        pub username: String,
        pub password: String,
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

    pub fn login(user: LoginUser, conn: Connection) -> Option<User> {
        match fetch(user.username, conn) {
            Some(database_user) => {
                if user.password.eq(&database_user.password) {
                    Some(database_user)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn fetch_by_id(id: i32, conn: Connection) -> Option<User> {
        let query = users::table.find(id).first::<User>(&*conn);

        match query {
            Ok(user) => Some(user),
            Err(diesel::result::Error::NotFound) => None,
            Err(_) => None,
        }
    }

    pub fn fetch(username: String, conn: Connection) -> Option<User> {
        let query = users::table
            .filter(users::columns::username.eq(username))
            .first::<User>(&*conn);

        match query {
            Ok(user) => Some(user),
            Err(diesel::result::Error::NotFound) => None,
            Err(_) => None,
        }
    }

    pub fn fetch_following(username: String) -> Option<Following> {
        Some(Following {
            username,
            following: vec![
                User {
                    id: 1,
                    username: "ghost".to_string(),
                    password: "password".to_string(),
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
                    id: 1,
                    username: "ghost".to_string(),
                    password: "password".to_string(),
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
