#[macro_use]
extern crate rocket;

use rocket::{form::Form, tokio::fs::read_to_string};
use rocket::response::content::RawHtml;

use std::{fs::OpenOptions, io::Write};

#[derive(FromForm)]
struct PostMsg {
    content: String,
}

#[post("/chat", data = "<msg>")]
async fn chat(msg: Form<PostMsg>) -> RawHtml<String> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("chat.txt")
        .unwrap();
    writeln!(file, "[anonymous]: {}<br>", msg.content).unwrap();

    return RawHtml(format!(
        "{}{}",
        // msg.content,
        read_to_string("chat.txt").await.unwrap(),
        include_str!("../html/chat.html")
    ));
}

#[get("/")]
async fn index() -> RawHtml<String> {
    return RawHtml(format!(
        "{}{}",
        // msg.content,
        read_to_string("chat.txt").await.unwrap(),
        include_str!("../html/chat.html")
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![chat])
}
