// use rocket::{get, launch, routes};
use rocket::{get, post, put, delete, routes};
// use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{json, Value};
// use rocket::serde::json::{Json, Value};


#[get("/")]
async fn hello() -> String {
    "hello world!".to_string()
}


// offical launch method

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/hello", routes![hello])
// }


// restful
// get method

#[get("/test")]
async fn test_get() -> String {
    "test get method".to_string()
}

// post method
#[post("/test")]
async fn test_post() -> String {
    "test post method".to_string()
}

// _ 消除告警
//put method
#[put("/test/<_id>")]
async fn test_put(_id: usize) -> String {
    "test put method".to_string()
}

//delete method
#[delete("/test/<_id>")]
async fn test_delete(_id: usize) -> String {
    "test delete method".to_string()
}

// restful get return json
#[get("/test_json")]
async fn test_get_json() -> Value {
    json!({"test": "test_get"})
}


#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/hello", routes![hello])
        .mount("/test", routes![test_get, test_post, test_put, test_delete])
        .mount("/test_json", routes![test_get_json])
        .launch().await?;
    Ok(())
}