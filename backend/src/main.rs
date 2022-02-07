#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod responders;
mod users;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/users",
        routes![
            users::controller::get,
            users::controller::get_all,
            users::controller::create
        ],
    )
}
