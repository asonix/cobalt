extern crate rocket;
extern crate rocket_cors;

use database;

mod authentication;
mod user;

pub fn create() -> Vec<rocket::Route> {
    routes!(
        authentication::register,
        authentication::login,
        authentication::me,
        authentication::logout,
        user::handler,
        user::inbox::handler,
        user::outbox::handler,
        user::following::handler,
        user::followers::handler
    )
}

pub fn create_cors_options() -> rocket_cors::Cors {
    rocket_cors::Cors {
        allow_credentials: true,
        ..Default::default()
    }
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for database::Connection {
    type Error = ();

    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<database::Connection, ()> {
        let pool = request.guard::<rocket::State<database::Pool>>()?;

        match pool.get() {
            Ok(connection) => rocket::Outcome::Success(database::Connection(connection)),
            Err(_) => rocket::Outcome::Failure((rocket::http::Status::ServiceUnavailable, ())),
        }
    }
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for database::users::User {
    type Error = ();

    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<database::users::User, ()> {
        let pool = request.guard::<rocket::State<database::Pool>>()?;

        match pool.get() {
            Ok(connection) => {
                let conn = database::Connection(connection);

                match request.cookies().get_private("user_id") {
                    Some(user_id) => {
                        match user_id.value().parse::<i32>() {
                            Ok(user_id) => {
                                match database::users::fetch_by_id(user_id, conn) {
                                    Some(user) => rocket::Outcome::Success(user),
                                    None => rocket::Outcome::Failure(
                                        (rocket::http::Status::NotFound, ()),
                                    ),
                                }
                            }
                            Err(_) => rocket::Outcome::Failure(
                                (rocket::http::Status::BadRequest, ()),
                            ),
                        }
                    }
                    None => rocket::Outcome::Failure((rocket::http::Status::BadRequest, ())),
                }
            }
            Err(_) => rocket::Outcome::Failure((rocket::http::Status::ServiceUnavailable, ())),
        }
    }
}
