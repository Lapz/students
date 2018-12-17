#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

extern crate jsonwebtoken as jwt;

mod auth;
mod models;
mod routes;
mod schema;
mod sql_pool;
mod users;
use self::sql_pool::init;
use dotenv::dotenv;
use rocket::response::Redirect;
use std::env;

#[get("/")]
fn root() -> &'static str {
    "Hi you have hit the api"
}

#[get("/a")]
fn redirect() -> Redirect {
    Redirect::to(uri!(root))
}

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATA_BASEURL must be set");
    rocket::ignite()
        .manage(init(&db_url))
        .mount("/", routes![routes::users::login, routes::users::create])
        .mount(
            "/api",
            routes![
                routes::students::students,
                routes::students::student,
                routes::students::add_student
            ],
        )
        .mount(
            "/api",
            routes![
                routes::classes::class,
                routes::classes::classes,
                routes::classes::class_grades,
                routes::classes::add_class
            ],
        )
        .launch();
}
