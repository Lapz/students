// A SQL connection pool. Taken from https://github.com/ghotiphud/rust-web-starter/blob/master/api_server/src/pg_pool.rs
// Copyright (c) 2017 Yale Cason III

use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;

type ManagedSqlConnection = ConnectionManager<MysqlConnection>;
pub type Pool = r2d2::Pool<ManagedSqlConnection>;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

/// Db Connection request guard type: wrapper around r2d2 pooled connection
pub struct DbConn(pub r2d2::PooledConnection<ManagedSqlConnection>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<'_, Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &PgConnection.
impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn init(database_url: &str) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}
