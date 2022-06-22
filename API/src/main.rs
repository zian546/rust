use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use color_eyre::Result;
use database::models::User;
use server_config::Config;
use tracing::info;

mod server_config;

#[actix_web::main]
async fn main() -> Result<()> {
    info!("loading config into server...");
    let config = Config::from_env().expect("loading server configurations");
    info!("server config loaded successfully.");
    
    info!("starting server at  host : {}, port : {}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(get_user)
            .route("/", web::get().to(home))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;
    info!("server started at  host :{}, port :{}", config.host, config.port);
    Ok(())
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body(String::from("Home Page"))
}

#[get("/user")]
async fn get_user() -> impl Responder {
    let connection = database::establish_connection();
    let another_user = database::find_all("user".to_string(), &connection);

    HttpResponse::Ok().json(another_user)
}
