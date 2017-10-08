extern crate rocket;
extern crate rocket_contrib;

use database;

#[post("/register", data = "<user>", format = "application/json")]
pub fn register(
    user: rocket_contrib::Json<database::users::RegisterUser>,
    conn: database::Connection,
) {
    database::users::register(user.0, conn);
}

#[post("/login", data = "<user>", format = "application/json")]
pub fn login(
    user: rocket_contrib::Json<database::users::LoginUser>,
    conn: database::Connection,
    mut cookies: rocket::http::Cookies,
) -> Option<rocket_contrib::Json<database::users::User>> {
    match database::users::login(user.0, conn) {
        Some(user) => {
            cookies.add_private(rocket::http::Cookie::new("user_id", user.id.to_string()));
            Some(rocket_contrib::Json(user))
        }
        None => None,
    }
}

#[get("/me", format = "application/json")]
pub fn me(user: database::users::User) -> rocket_contrib::Json<database::users::User> {
    rocket_contrib::Json(user)
}

#[post("/logout")]
pub fn logout(mut cookies: rocket::http::Cookies) {
    match cookies.get_private("user_id") {
        Some(cookie) => cookies.remove_private(cookie),
        None => {}
    }
}
