#[derive(Queryable)]
pub struct User {
    // Order must match db/schemas.rs
    pub id: i32,
    pub username: String,
}
