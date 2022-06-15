use std::io;
pub struct User{
    username: String,
    email: String,
    phone: String,
    activation_status:bool
}

pub fn user_factory(
    username: String ,
    email : String ,
    phone : String,
    activation_status: bool) -> User{

       let temp_user= User{
            username,
            email,
            phone,
            activation_status: activation_status
        };

        return temp_user;
    }

pub fn main() {

    let mut username = String::new();

    io::stdin()
    .read_line(&mut username)
    .ok()
    .expect("error reading username");

    

    



}