#[macro_use]
extern crate rocket;

mod responders;
use responders::ok_responders::{OkJson, OkJsonResponse};
use rocket::serde::json::json;

#[get("/")]
fn index() -> OkJsonResponse {
    return Result::Ok(OkJson(json!({
        "data": "Hello, World!",
    })));
}

#[get("/error")]
fn error() -> OkJsonResponse {
    return Result::Err("This request always fails!");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, error])
}
