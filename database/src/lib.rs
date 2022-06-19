#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;
pub mod models;



use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::*;
use self::schema::user::dsl::*;


pub fn establish_connection()-> MysqlConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL cannot be empty");
    MysqlConnection::establish (&database_url)
        .expect(&format!("Error connecting to database '{}'", database_url))
}

pub fn searh_by_id( _id: i32, con: &MysqlConnection) -> Vec<User>{
    let result = user
        .filter(id.eq(_id))
        .limit(1)
        .load::<User>(con)
        .expect("Error loading user from database");

    return result
}

pub fn search_by_username( _username: String,  con: &MysqlConnection) -> Vec<User> {
    let result = user
    .filter(username.eq(_username))
    .limit(1)
    .load::<User>(con)
    .expect("Error loading user from database");

return result
}

pub fn register_new_user( _username: String, _password: String,  con: &MysqlConnection) -> bool{



    let temp_user = NewUser{
        username : _username,
        password : _password,
    };

    //the value that will be returned by this function, marking completion with true as succes and false if it fails for some reason
    let mut return_val:bool = true;

    //insert user into database
    diesel::insert_into(user)
        .values(&temp_user)
        .execute(con)
        .expect( "error inserting user into database")
        .get_result(con)
        .unwrap();

        return return_val
}

pub fn update_user_by_id(_id: i32,con: &MysqlConnection) -> bool { }

pub fn delete_user_by_id(_id: i32,con: &MysqlConnection) -> bool { }

pub fn delete_user_by_username(_username: String, con: &MysqlConnection) -> bool { }


