#[macro_use]
extern crate rocket;

use rocket::response::content::RawHtml;
use rocket::{form::Form, tokio::fs::read_to_string};

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
    if !msg.content.is_empty() {
        writeln!(file, "<div class=\"msgbox\"><div class=\"username\">[anonymous]</div> {}</div><br>", msg.content).unwrap();
    }
    return RawHtml(format!(
        "{}{}",
        // msg.content,
        read_to_string("chat.txt").await.unwrap(),
        read_to_string("html/chat.html").await.unwrap()
    ));
}

#[get("/")]
async fn index() -> RawHtml<String> {
    return RawHtml(format!(
        "{}{}",
        // msg.content,
        read_to_string("chat.txt").await.unwrap(),
        read_to_string("html/chat.html").await.unwrap()
    ));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![chat])
}
