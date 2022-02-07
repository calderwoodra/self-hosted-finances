use rocket::serde::json::Value;

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct OkJson(pub Value);

pub type OkJsonResponse = Result<OkJson, &'static str>;
