use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyResponse {
    message: String,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    let response = MyResponse {
        message: "Hello Joe!".to_string(),
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fs::Files::new("/", "../frontend/public").index_file("index.html"))
    })
    .bind("0.0.0.0:3080")?
    .run()
    .await
}
