///this is the model that will be used to interact with the database

use crate::schema::user;
use diesel::Insertable;
use diesel::QueryableByName;
use serde::{Serialize, Deserialize};

/// This is the struct used to get data from the database
/// ```
/// pub struct User{
/// pub id: i32,
/// pub username: String,
/// pub password: String,
/// pub value: i32
/// }
/// ```
/// it contains the basic information abput the user that have been saved inside the database
#[derive(Serialize ,QueryableByName, PartialEq, Debug)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub value: i32
}


/// while this is the struct that we will be using to insert NEW user in the database
/// ```
/// pub struct NewUser  <'a>   {
/// pub username: String,
/// pub password: String,
///}
/// ```
#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "user"]
pub struct NewUser     {
    pub username: String,
    pub password: String,
}
