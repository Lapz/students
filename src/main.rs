#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_contrib;

extern crate jsonwebtoken as jwt;

mod auth;
mod models;
mod routes;
mod schema;
mod sql_pool;

use crate::sql_pool::init;
use dotenv::dotenv;
use rocket::response::Redirect;
use std::env;
use rocket::http::Method;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

#[get("/")]
fn root() -> Template {
    let context:HashMap<(),()> = HashMap::new();
    Template::render("login",&context)
}

#[get("/a")]
fn redirect() -> Redirect {
    Redirect::to(uri!(root))
}

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATA_BASEURL must be set");
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000/"]);
    assert!(failed_origins.is_empty());

    // You can also deserialize this
    let cors = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .attach(cors)
        .attach(Template::fairing())
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
