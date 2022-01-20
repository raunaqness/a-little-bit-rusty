#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

use crate::schema::todo;
use rocket::{self, get, post, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};

mod schema;

#[database("postgres")]
struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
struct Todo {
    id: i32,
    title: String,
    checked: bool
}

#[derive(Insertable, Deserialize)]
#[table_name="todo"]
struct NewTodo {
    title: String
}

#[get("/")]
fn get_todos(conn: DbConn) -> Json<Vec<Todo>> {
    let todos = todo::table
        .order(todo::columns::id.desc())
        .load::<Todo>(&*conn)
        .unwrap();
    Json(todos)

}

#[post("/", data = "<new_todo>")]
fn create_todo(conn: DbConn, new_todo: Json<NewTodo>) -> Json<Todo>{
    let result = diesel::insert_into(todo::table)
        .values(&new_todo.0)
        .get_result(&*conn)
        .unwrap();
    Json(result)

}

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
    .attach(DbConn::fairing())
    .mount("/voting", routes![voting])
    .mount("/todos", routes![get_todos, create_todo])
    .launch();
}
