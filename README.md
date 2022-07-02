# Rust simple web server template
This is a repo that aims to make developing web server a little bit easier. It has all the basic starting boiler plate code
and some example to get you started writing API's right away and not worry about the hassle of writing the boiler plate code
required to run the web server. 

## What to expect
This template only provide the basics of the basic web server boiler plate code. by that i mean  *__no security__* whatsoever(except __cors__).
you have to implement the security and error handling yourself. you *__can technically__* run this  right away
once you cloned it. but i wouldn't reccomend it *especially* if you have an intention to use this in a production environment 
due to security concerns.

## What inside 
This template uses __actix__ framework as the web server and __diesel__ as the orm mapping to  __mysql__. this template is intended
to be used with a sql database. if you want to use this template for other databases such as __mongodb__ you have to write
the database interaction logic from scratch. incase of __mongodb__, they actually have an official crate maintained by the 
mongodb team([here](https://crates.io/crates/mongodb) or [here](https://docs.rs/mongodb/2.2.2/mongodb/)). although i still reccomends
to write the database interaction logic separate from the api to improve project maintainability.


This template has 2 sections on it, the first section is where you wil be building your web server endpoint([here](https://github.com/zian546/web-server-rust-backbone/tree/main/API)).
the next section is where your database interaction logic will be written([here](https://github.com/zian546/web-server-rust-backbone/tree/main/database/src)). this will make your project alot easier to maintain
because the logic and the api is seperate from eachother.

## :bangbang: IMPORTANT

dont' forget to create a ```.env``` file with a ```DATABASE_URL```, ```HOST```, and a ```PORT``` field or else it would throw  
a compiler error because this template will merge with your ```.env``` file. (see [here](https://github.com/zian546/web-server-rust-backbone/blob/main/API/src/server_config/mod.rs))
you can change and modify the ```Config``` struct based on your needs. you'll just have to call the ```from_env``` function 
to load your ```.env``` variable to your source code.

---
### Example:
this will load the *__.env variable__* to your soruce code.
```rust
let config = Config::from_env().expect("loading server configurations");
```
then you can just acces the variable like this
```rust
let port = config.port // this will load the value that have been assigned to the PORT variable in the .env file
```
---

Right now this template only support making new connection to the database every *request*. this is terrible and i'm planning
to improve this later(PR is welcomed). don't use the [database_connecion](https://github.com/zian546/web-server-rust-backbone/blob/main/API/src/database_connection/mod.rs) module because it's still work in progress.
use the [establish_connection](https://github.com/zian546/web-server-rust-backbone/blob/main/database/src/lib.rs) method from the database crate instead.
### Example:
```rust
let db_connection = database::establish_connection();
```

### Misc
<li>future task:</li><br/>

- [ ] pool database connection

## Contact:
- Email: <mynameiszian@gmail.com>:email:

  


