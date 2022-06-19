extern crate diesel;

use diesel::Queryable;
use diesel::Insertable;
use crate::schema::user;

//model for instering user to database at registration
#[derive(Queryable)]
pub struct User{
    pub id: u32,
    pub username:String,
    pub password:String
}

#[derive(Insertable)]
#[table_name ="user"]
pub struct Newuser {
    pub username:String,
    pub password:String
}

pub fn main(){

}