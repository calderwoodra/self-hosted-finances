use crate::db::db;
use crate::db::schema;
use crate::diesel::prelude::*;
use crate::responders::ok_responders::{OkJson, OkJsonResponse};
use crate::users::user_model::User;
use rocket::serde::json::json;

#[get("/")]
pub fn get_user() -> OkJsonResponse {
    let users = users();
    return Result::Ok(OkJson(json!({
        "user": "User...",
    })));
}

fn users() {
    use crate::db::schema::users::dsl::*;

    let connection = db::establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("----------\n");
        println!("{}", user.username);
    }
}
