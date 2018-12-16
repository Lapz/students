use ring::{digest, pbkdf2};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};
use std::env;

pub static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
pub const DB_SALT_COMPONET: [u8; 16] = [
    0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52, 0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a,
];

pub const NUMBER_ITERATIONS: u32 = 100_000;

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];
use crate::sql_pool::Pool;

use rocket::http::RawStr;
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey(pub String);

#[derive(FromForm)]
pub struct UserLogin {
    pub username: String,
    pub password: String
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();

        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        match jwt::decode::<ApiKey>(
            &keys[0],
            &env::var("JWT_SECRET")
                .expect("Secret should be set")
                .as_bytes(),
            &jwt::Validation::default(),
        ) {
            Ok(claim) => Outcome::Success(claim.claims),
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
