#[macro_use]
extern crate rocket;

use rocket::response::content::RawHtml;
use rocket::{form::Form, tokio::fs::read_to_string};

use std::{fs::OpenOptions, io::Write};

#[derive(FromForm)]
struct PostMsg {
    content: String,
}

#[post("/chat/<username>", data = "<msg>")]
async fn chat(username: String, msg: Form<PostMsg>) -> RawHtml<String> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("chat.txt")
        .unwrap();
    let _stamp = chrono::offset::Utc::now().to_string();
    let timestamp = _stamp.split(".").next().unwrap();
    if !msg.content.is_empty() {
        writeln!(
            file,
            "<span class=\"timestamp\">{timestamp}</span> <span class=\"{username}-username username\">[{username}]</span> <a class=\"{username}-msg msgbox\">{}</a><br>",
            msg.content
        )
        .unwrap();
    }
    return RawHtml(format!(
        "<div class=\"chatbox\"><div class=\"title\">Hackchat.exe</div><br>{}{}</div>",
        // msg.content,
        read_to_string("chat.txt").await.unwrap(),
        read_to_string("html/chat.html").await.unwrap()
    ));
}

#[get("/chdata")]
async fn chdata() -> String {
    read_to_string("chat.txt").await.unwrap()
}

#[get("/login")]
async fn login() -> RawHtml<String> {
    return RawHtml(format!(
        "{}",
        read_to_string("html/login.html").await.unwrap()
    ));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![login])
        .mount("/", routes![chat])
        .mount("/", routes![chdata])
}
