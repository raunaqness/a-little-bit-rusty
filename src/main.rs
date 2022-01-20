#![feature(decl_macro)]

use rocket::{self, get, routes};

#[get("/vote/<name>/<age>")]
fn voting(name: String, age: u8) -> String {
    if age > 18{
        format!("{} can vote!", name)
    } else {
        format!("come back when you're 18.")
    }
}

fn main() {
    rocket::ignite()
    .mount("/", routes![voting])
    .launch();
}
