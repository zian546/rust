#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod models;
pub mod schema;

use self::models::*;
use self::schema::user::dsl::*;
use diesel::sql_types::Integer;
use diesel::{connection, prelude::*, sql_query};
use dotenv::dotenv;
use std::env;

//establish connection to database
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to database '{}'", database_url))
}

//find user by id in database
pub fn is_exist_by_id(_id: i32, connection_to_database: &MysqlConnection) -> Vec<User> {
    let result = sql_query(format!("SELECT * FROM `user` WHERE `id` = '{}' ", _id))
        .load::<User>(connection_to_database)
        .unwrap();

    return result;
}

//find user by username in database
pub fn is_exist_by_username(
    _username: String,
    connection_to_database: &MysqlConnection,
) -> Vec<User> {
    let result = sql_query(format!(
        "SELECT * FROM `user` WHERE `username` = '{}'",
        _username
    ))
    .load::<User>(connection_to_database)
    .unwrap();

    return result;
}

pub fn register_new_user(
    _username: String,
    _password: String,
    connection_to_database: &MysqlConnection,
) {
    let temp_user = NewUser {
        username: &_username,
        password: &_password,
    };

    //insert user into database
    let result = diesel::insert_into(user)
        .values(&temp_user)
        .execute(connection_to_database)
        .unwrap();
}

/*
pub fn update_user_by_id(_value:i32,_id: i32,con: &MysqlConnection) -> bool {}

pub fn delete_user_by_id(_id: i32,con: &MysqlConnection) -> bool { }

pub fn delete_user_by_username(_username: String, con: &MysqlConnection) -> bool { }


pub fn hash_password(_password: String) -> String{

}
*/

pub fn main() {
    let connection_db = establish_connection();

    let user_exist = is_exist_by_id(1, &connection_db);

    println!("{:?}", user_exist)
}
