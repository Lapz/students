use rocket::request::{self,FromRequest};
use rocket::{Outcome, Request};
use rocket::http::Status;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey(pub String);



impl <'a,'r> FromRequest<'a,'r> for ApiKey {
    type Error = ();

    fn from_request(request:&'a Request<'r>) -> request::Outcome<ApiKey,()> {
        let keys:Vec<_> = request.headers().get("Authentication").collect();

        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest,()));
        }


        match jwt::decode::<ApiKey>(&keys[0],&env::var("JWT_SECRET").expect("Secret should be set").as_bytes(),&jwt::Validation::default()) {
            Ok(claim) => Outcome::Success(claim.claims),
            Err(_) => Outcome::Failure((Status::Unauthorized,()))
        }
    }
}