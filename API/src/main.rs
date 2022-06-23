use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use color_eyre::Result;
use database::models::User;
use server_config::Config;
use tracing::info;
use database;

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
            .service(create_user)
            .route("/", web::get().to(home))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;
    info!("server started at  host :{}, port :{}", config.host, config.port);
    Ok(())
}

async fn home() -> impl Responder{
    HttpResponse::Ok().body("Home page".to_string())
}

#[get("/user")]
async fn get_user() -> impl Responder{
    let db_connection = database::establish_connection();

    let result = database::find_all("`user`".to_string(),&db_connection,"".to_string());

    HttpResponse::Ok().json(result)
}

#[post("/create_user")]
async fn create_user(new_user: web::Json<database::models::NewUser>) -> impl Responder{
    let username = new_user.username.clone();
    let password = new_user.password.clone();
    let db_connection = database::establish_connection();
    let result = database::register_new_user( username, password, &db_connection);

    HttpResponse::Ok()//.body(result.to_string())
}
