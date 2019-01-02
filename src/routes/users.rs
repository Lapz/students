use crate::auth::{ApiKey, Claims, UserLogin, NUMBER_ITERATIONS};

use crate::models::NewUser;
use crate::sql_pool::Pool;
use chrono::Utc;
use diesel::prelude::{Connection, ExpressionMethods, Insertable, QueryDsl, RunQueryDsl};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::JsonValue;
use std::env;
use rocket::{Outcome, Request};

#[post("/auth", data = "<user>")]
pub fn login(
    user: Form<UserLogin>,
    db_pool: State<'_, Pool>,
) -> Result<JsonValue, JsonValue> {
    use crate::schema::users;

    let res = users::table
        .filter(users::username.eq(user.username.as_str()))
        .select(users::password)
        .first::<String>(&db_pool.inner().get().expect("no connection"));

    match res {
        Ok(actual_password) => {
            if let Err(_) = pbkdf2::pbkdf2_check(&user.password, &actual_password) {

                Err(json!({
                "msg":"Invalid password",
                "status":401
            }))
                
            } else {
                let username = user.into_inner().username;

                let claim = Claims {
                    username,
                    exp: Utc::now().timestamp() + 10000,
                };

                let token = jwt::encode(
                    &jwt::Header::default(),
                    &claim,
                    &env::var("JWT_SECRET")
                        .expect("Secret should be set")
                        .as_bytes(),
                )
                .unwrap();

                Ok(json!({ "key": ApiKey(token),"status":200 }))
            }
        }
        
        Err(e) => {
            println!("{}",e);
            Err(json!({
                "msg":"Unknown User",
                "status":401
            }))
        },
    }
}

#[post("/auth/create", data = "<user>")]
pub fn create(user: Form<UserLogin>, db_pool: State<'_, Pool>) -> String {
    use crate::schema::users;

    let user = user.into_inner();
    let username = user.username;
    let password = pbkdf2::pbkdf2_simple(&user.password, NUMBER_ITERATIONS).unwrap();

    let user = NewUser { username, password };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(&db_pool.inner().get().expect("no connection"))
        .expect("Error loading posts");

    "User Added".into()
}