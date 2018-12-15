use crate::auth::{ApiKey, UserLogin};
use crate::models::Student;
use rocket::request::Form;
use rocket_contrib::json::Json;
use diesel::prelude::*;
use crate::models::User;
use crate::sql_pool::Pool;
use rocket::State;
use auth::DB_SALT_COMPONET;

#[get("/login", data = "<user>")]
pub fn login(user: Form<UserLogin>,db_pool:State<Pool>) {
    

    use crate::schema::users;

    let username = users::table
        .filter(users::username.eq(user.username))
        .first::<User>(&db_pool.inner().get().expect("no connection"));


    match username {
        Ok(res) => {
            let mut salt = Vec::with_capacity(DB_SALT_COMPONET.len()+user.username.len());

        },
        Err(_) => {
            "unkonw user";
        }
    }
    // let res = users::table
    // .filter(users::username.eq(&user.username));

    // .filter()

    // Err(())
}
