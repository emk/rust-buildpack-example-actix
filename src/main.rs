use actix_web::{get, web, App, HttpServer, Responder};

use std::env;

#[get("/{name}")]
async fn index(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port:u32 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| App::new().service(index))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}