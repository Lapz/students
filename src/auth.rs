
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
        let mut cookies = request.cookies();




        if let Some(cookie) = cookies.get_private("api") {
            match jwt::decode::<Claims>(
            &cookie.value(),
            &env::var("JWT_SECRET")
                .expect("Secret should be set")
                .as_bytes(),
            &jwt::Validation::default(),
        ) {
            Ok(claim) => Outcome::Success(ApiKey(claim.claims.username)),
            Err(_) => {
                Outcome::Failure((Status::Unauthorized, ()))
            },
        }
        }else {
            Outcome::Failure((Status::BadRequest, ()))
        }
    }
}
