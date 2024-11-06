mod aws_handler;

use actix_web::{get, App, HttpServer, Responder};
use aws_handler::AwsHandler;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

#[get("/health")]
async fn health() -> impl Responder {
    "I'm alive!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let aws_handler = AwsHandler::new().await.expect("Failed to create AWS handler");

    // Example usage of AWS handler
    aws_handler.list_buckets().await.expect("Failed to list buckets");

    HttpServer::new(|| App::new().service(hello).service(health))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}