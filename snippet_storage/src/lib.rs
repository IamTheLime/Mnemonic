#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection_sqlite() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("PLEASE SET DABASE_URL environment variable");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error when connecting to db: {}", database_url))

}

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("PLEASE SET DABASE_URL environment variable");

    PgConnection::establish(&database_url)
        .expect(&format!("Error when connecting to db: {}", database_url))








}
