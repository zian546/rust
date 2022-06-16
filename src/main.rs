use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_hello_service)
            .route("/hello", web::get().to(get_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body(String::from("Hello, world!"))
}

#[get("/hello_service")]
async fn get_hello_service() -> impl Responder {
    HttpResponse::Ok().body(String::from("Hello, world! From service"))
}
