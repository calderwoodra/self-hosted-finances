use rocket::serde::Deserialize;

use crate::db::schema::users;

#[derive(Queryable)]
pub struct User {
    // Order must match db/schemas.rs
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateUserBody<'r> {
    pub username: &'r str,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
}
