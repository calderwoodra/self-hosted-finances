use crate::db::db;
use crate::db::schema::users;
use crate::diesel::prelude::*;
use crate::responders::ok_responders::{OkJson, OkJsonResponse};
use crate::users::user_model::{CreateUserBody, NewUser, User};
use rocket::serde::json::{json, Json};

// curl -X GET http://127.0.0.1:8000/users/
#[get("/")]
pub fn get_all() -> OkJsonResponse {
    let connection = db::establish_connection();
    let users = get_all_users(&connection);
    return Result::Ok(OkJson(json!({
        "user": format!("{} users exist", users.len()),
    })));
}

fn get_all_users(connection: &PgConnection) -> Vec<User> {
    return users::table
        .load::<User>(connection)
        .expect("Error loading users");
}

// curl -X GET http://127.0.0.1:8000/users/1/
#[get("/<id>")]
pub fn get(id: i32) -> OkJsonResponse {
    let connection = db::establish_connection();
    let user = get_user(&connection, id);
    return Result::Ok(OkJson(json!({
        "user": {
            "id": user.id,
            "username": user.username,
        },
    })));
}

fn get_user(connection: &PgConnection, id: i32) -> User {
    return users::table
        .filter(users::id.eq(id))
        .get_result::<User>(connection)
        .expect("Error loading users");
}

// curl -X POST -d '{"username":"awsick"}' http://127.0.0.1:8000/users/create
#[post("/create", data = "<user>")]
pub fn create(user: Json<CreateUserBody<'_>>) -> OkJsonResponse {
    let connection = db::establish_connection();
    let result = create_user(&connection, user.username);
    return Result::Ok(OkJson(json!({
        "user": {
            "id": result.id,
            "username": result.username,
        },
    })));
}

pub fn create_user(connection: &PgConnection, name: &str) -> User {
    let new_user = NewUser { username: name };

    return diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error creating new user");
}
