#[macro_use]
extern crate diesel;
extern crate dotenv;

///this is the models that i built that contains the models to interact with the database using Diesel's ORM
pub mod models;
///while this the schema of the database that provide us the table name that we needed when executing a ```sql_query```
pub mod schema;

use self::models::*;
use self::schema::user::dsl::*;
use diesel::{delete, insert_into, prelude::*, sql_query, update};
use dotenv::dotenv;
use std::env;

/// this function establish connection to database.
///
pub fn establish_connection() -> MysqlConnection {
    //! this function is the *main* core of this library, it's job is to establish a connection to the database
    //! and allows us to do all sorts of things with the database
    //!
    //! #Code:
    //! ```
    //! dotenv().ok();
    //!
    //! let database_url = env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    //! MysqlConnection::establish(&database_url)
    //!      .expect(&format!("Error connecting to database '{}'", database_url))
    //! ```
    //!
    //! this loads the ```.env``` file to be used in this function.
    //! ```
    //! dotenv().ok()
    //! ```
    //!
    //! this actually save the url of the database inside the ```database_url``` variable
    //! ```
    //! let database_url = env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    //! ```
    //! and this attempt a connection to the database using ```MysqlConnection::establish``` method.
    //! ```
    //!  MysqlConnection::establish(&database_url)
    //!      .expect(&format!("Error connecting to database '{}'", database_url))
    //! ```

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to database '{}'", database_url))
}

/// this function retrieve user from database using id as a parameter.
///
pub fn find_by_id(_id: i32, connection_to_database: &MysqlConnection) -> Vec<User> {
    //! this function returns a Vector of ```<User>``` type
    //! even if it couldn't find the desired user using id so be sure to pay attention to the return value,
    //! this means that if the function couldn't find the desired user, then it will only return an empty vector.
    //!
    //! #Code:
    //!
    //! ```
    //! let result = sql_query(format!(
    //!     "SELECT * FROM `user` WHERE `id` = '{} LIMIT 1' ",
    //!     id
    //!   ))
    //! .load::<User>(connection_to_database)
    //! .unwrap();
    //! ```
    //!
    //! this code below construct the SQL Query  
    //! ```
    //! let result = sql_query(format!(
    //!    "SELECT * FROM `user` WHERE `id` = '{} LIMIT 1' ",
    //!     id
    //!   ))
    //! ```
    //! 
    //! the load method retirieve the query result and store it inside the ```<User>``` Vector to be returned
    //! when the function is called.
    //! 
    //! ```
    //! .load::<User>(connection_to_database)
    //! ```
    //! 
    //! while the ```.unwrap()``` method is used to literally _unwrap_ the ```result``` from ```Result<Vec<User>,Error>``` to just ```Vec<User>``` by consuming the ```Error``` result
    //! leaving just the vector to be returned.

    let result = sql_query(format!(
        "SELECT * FROM `user` WHERE `id` = '{} LIMIT 1' ",
        _id
    ))
    .load::<User>(connection_to_database)
    .unwrap();

    return result;
}

/// this function retrieve user from database using ```username: String``` as a parameter.
///
pub fn find_by_username(_username: String, connection_to_database: &MysqlConnection) -> Vec<User> {
    //! this function also works similarly compared to ```pub fn find_by_id()``` which means that
    //! this function also returns an empty vector if it didn't find the desired user
    //! you only have to pass in the username(```String NOT &str```).
    //!
    //! #Code:
    //!
    //! ```
    //! let result = sql_query(format!(
    //!    "SELECT * FROM `user` WHERE `username` = '{}' LIMIT 1",
    //!    _username
    //! ))
    //! .load::<User>(connection_to_database)
    //! .unwrap();
    //!
    //! return result;
    //! ```
    //! in this function we don't need to use the ```unwrap_or_else()``` method because even if the query didn't find anything, it _STILL_ return something and that is
    //! an empty vector of type ```Vec<User>```.

    let result = sql_query(format!(
        "SELECT * FROM `user` WHERE `username` = '{}' LIMIT 1",
        _username
    ))
    .load::<User>(connection_to_database)
    .unwrap();

    return result;
}

///this function return a vector type```Vec<User>``` and will return all the user that are currently saved in the database.
pub fn find_all(connection_to_database: &MysqlConnection) -> Vec<User> {
    let result = sql_query("SELECT * FROM `user` ")
        .load::<User>(connection_to_database)
        .unwrap();

    return result;
}

/// this function save  a user in database
pub fn register_new_user(
    _username: String,
    _password: String,

    connection_to_database: &MysqlConnection,
) -> bool {
    //! this is the struct that we're gonna use to save user to the database.
    //! ```
    //! let temp_user = NewUser{
    //! username: &_username,
    //! password: &_password
    //! };
    //! ```
    //! pay attention to the struct, it is the same model that we
    //! defined in the ```model.rs``` file
    //! this is important because the model contains the fields that we must specify in the struct
    //! so that the diesel knows where to put what.
    //! while this is what we actually DO to insert user into the database
    //! ```
    //! let result = insert_into(user)
    //!     .values(&temp_user);
    //!     .execute(connection_to_database)
    //!     .unwrap_or_else(|err| return 0);
    //! ```
    //! notice that in the ```insert_into``` statement, we specify the table name. this possible because we are using the
    //! schema that provide us what table we have in the database.
    //! also note that we use the ```.unwrap_or_else``` method.
    //! this is because we want to avoid the thread stopping completely when something goes wrong at the time
    //! we run this function.

    let temp_user = NewUser {
        username: &_username,
        password: &_password,
    };

    let result = insert_into(user)
        .values(&temp_user)
        .execute(connection_to_database)
        .unwrap_or_else(|err| return 0);

    if result == 0 {
        return false;
    } else {
        return true;
    }
}

//update user that existed in database
pub fn update_user_by_id(_id: i32, _value: i32, con: &MysqlConnection) -> bool {
    let result = update(user)
        .filter(id.eq(_id))
        .set(value.eq(_value))
        .execute(con)
        .unwrap_or_else(|err| return 0);

    if result == 0 {
        return false;
    } else {
        return true;
    }
}

pub fn update_user_by_username(_username: String, _value: i32, con: &MysqlConnection) -> bool {
    let result = update(user)
        .filter(username.eq(_username))
        .set(value.eq(_value))
        .execute(con)
        .unwrap_or_else(|err| return 0);

    if result == 0 {
        return false;
    } else {
        return true;
    }
}

//delete user in database
pub fn delete_user_by_id(_id: i32, con: &MysqlConnection) -> bool {
    let result = delete(user.filter(id.eq(_id)))
        .execute(con)
        .unwrap_or_else(|err| return 0);

    if result == 0 {
        return false;
    } else {
        return true;
    }
}

pub fn delete_user_by_username(_username: String, con: &MysqlConnection) -> bool {
    let result = delete(user.filter(username.eq(_username)))
        .execute(con)
        .unwrap_or_else(|err| return 0);

    if result == 0 {
        return false;
    } else {
        return true;
    }
}
