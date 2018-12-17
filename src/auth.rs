
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};
use std::env;

pub const NUMBER_ITERATIONS: u32 = 100_000;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey(pub String);

#[derive(FromForm)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub exp:i64
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();

        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        match jwt::decode::<Claims>(
            &keys[0],
            &env::var("JWT_SECRET")
                .expect("Secret should be set")
                .as_bytes(),
            &jwt::Validation::default(),
        ) {
            Ok(claim) => Outcome::Success(ApiKey(claim.claims.username)),
            Err(e) => {
                Outcome::Failure((Status::Unauthorized, ()))
            },
        }
    }
}
