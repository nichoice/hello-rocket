use rocket::get;
use rocket::serde::json::{json, Value};


#[get("/user")]
pub async fn get_users() -> Value {
    json!({"success": true, "data": []})
}