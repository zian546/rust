use crate::schema::user;

use diesel::Insertable;
use diesel::QueryableByName;

#[derive(QueryableByName, PartialEq, Debug)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub value: i32
}

//model for inserting user to database at registration
#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser  <'a>   {
    pub username: &'a str,
    pub password: &'a str,
}
