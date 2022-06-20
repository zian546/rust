#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod models;
pub mod schema;

use self::models::*;
use self::schema::user::dsl::*;
use diesel::{prelude::*, sql_query, update};
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
pub fn find_by_id(_id: i32, connection_to_database: &MysqlConnection) -> Vec<User> {
    let result = sql_query(format!("SELECT * FROM `user` WHERE `id` = '{}' ", _id))
        .load::<User>(connection_to_database)
        .unwrap();

    return result;
}

//find user by username in database
pub fn find_by_username(_username: String, connection_to_database: &MysqlConnection) -> Vec<User> {
    let result = sql_query(format!(
        "SELECT * FROM `user` WHERE `username` = '{}'",
        _username
    ))
    .load::<User>(connection_to_database)
    .unwrap();

    return result;
}

//find all user on the database
pub fn find_all(connection_to_database: &MysqlConnection) -> Vec<User> {
    let result = sql_query("SELECT * FROM `user` ")
        .load::<User>(connection_to_database)
        .unwrap();

    return result;
}

//register a user in database
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
    diesel::insert_into(user)
        .values(&temp_user)
        .execute(connection_to_database)
        .unwrap();
}

//update user that existed in database
pub fn update_user_by_id(_value: i32, _id: i32, con: &MysqlConnection)-> usize {
    let result = update(user)
        .filter(id.eq(_id))
        .set(value.eq(_value))
        .execute(con)
        .unwrap();

        return result
}

pub fn update_user_by_username(_username: String, _value: i32, con: &MysqlConnection,){
    
}

/*
pub fn delete_user_by_id(_id: i32,con: &MysqlConnection) -> bool { }

pub fn delete_user_by_username(_username: String, con: &MysqlConnection) -> bool { }


pub fn hash_password(_password: String) -> String{

}
*/

pub fn main() {
    let connection_db = establish_connection();
    let user_exist = find_all(&connection_db);

    println!("{:?}", user_exist);

    
    let update_user = update_user_by_id(30, 1, &connection_db);
    let user_exist = find_all(&connection_db);

    println!("{:?}", update_user);
}
