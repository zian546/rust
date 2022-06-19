#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;
pub mod models;



use diesel::prelude::*;
use dotenv::dotenv;
use std::env;


pub fn establish_connection()-> MysqlConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL cannot be empty");
    MysqlConnection::establish (&database_url)
        .expect(&format!("Error connecting to database '{}'", database_url))
}

pub fn main(){
    establish_connection();
}