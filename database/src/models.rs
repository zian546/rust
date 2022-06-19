use schema::user;

use diesel::Insertable;
use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
}

//model for inserting user to database at registration
#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser  {
    pub username: String,
    pub password: String,
}
