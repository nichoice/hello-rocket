// use rocket::{get, launch, routes};
use rocket::{get, routes};


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

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/hello", routes![hello])
        .launch().await?;
    Ok(())
}