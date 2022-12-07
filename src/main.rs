#[macro_use] extern crate rocket;
use std::time::SystemTime;
// Try visiting:
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/")]
fn wave() -> String {
    println!("{:?}", SystemTime::now());
    format!("Witaj, błaźnie!\nTest wielu wierszy.")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/wave", routes![wave])
}