use crate::responders::ok_responders::{OkJson, OkJsonResponse};
use rocket::serde::json::json;

#[get("/")]
pub fn index() -> OkJsonResponse {
    return Result::Ok(OkJson(json!({
        "data": "Hello, World!",
    })));
}

#[get("/error")]
pub fn error() -> OkJsonResponse {
    return Result::Err("This request always fails!");
}
