use crate::auth::*;
use crate::auth::{ApiKey, UserLogin};
use crate::models::Student;
use crate::models::User;
use crate::sql_pool::Pool;
use diesel::prelude::*;
use ring::pbkdf2;
use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::Json;
use std::env;
use rand::Rng;
use rand::distributions::Alphanumeric;

#[post("/auth", data = "<user>")]
pub fn login(user: Form<UserLogin>, db_pool: State<Pool>) -> Result<Json<ApiKey>, String> {
    use crate::schema::users;

    
    let res = users::table
        .filter(users::username.eq(user.username.as_str()))
        .select((users::password, users::salt))
        .first::<(String, String)>(&db_pool.inner().get().expect("no connection"));

    match res {
        Ok((actual_password, salt)) => {
            let mut secret: Vec<u8> =
                Vec::with_capacity(DB_SALT_COMPONET.len() + user.username.len());
            secret.extend(DB_SALT_COMPONET.as_ref());
            secret.extend(user.username.as_str().as_bytes());

            if let Err(_) = pbkdf2::verify(
                DIGEST_ALG,
                NUMBER_ITERATIONS,
                &secret,
                user.username.as_str().as_bytes(),
                actual_password.as_bytes(),
            ) {
                Err("invalid password".into())
            } else {
                #[derive(Serialize, Deserialize)]
                struct Claims<'a> {
                    username: &'a str,
                };

                let claim = Claims {
                    username: user.username.as_str(),
                };

                let token = jwt::encode(
                    &jwt::Header::default(),
                    &claim,
                    &env::var("JWT_SECRET")
                        .expect("Secret should be set")
                        .as_bytes(),
                )
                .unwrap();

                Ok(Json(ApiKey(token)))
            }
        }
        Err(_) => Err("unkonw user".into()),
    }
}

fn random_salt() -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(12).collect()
}


#[get("/auth", data = "<user>")]
pub fn create(user: Form<UserLogin>, db_pool: State<Pool>) -> String {
    use crate::schema::users;

    let salt = random_salt();

    "Err".into()
}
