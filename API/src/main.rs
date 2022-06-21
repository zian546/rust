use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use server_config::Config;
use database::models::User;
use color_eyre::Result;

mod server_config;



#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("loading server configurations"); 

    HttpServer::new(move ||
         App::new()
        .service(get_user)
        .route("/", web::get().to(home)))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
        Ok(())
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body(String::from("Home Page"))
}

#[get("/user")]
async fn get_user() -> impl Responder { 
    
    let connection = database::establish_connection();
let another_user = database::find_all("user".to_string(),&connection);

    HttpResponse::Ok().json(another_user)
}
