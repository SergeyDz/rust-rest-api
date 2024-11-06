mod aws_handler;

use actix_web::{get, web, App, HttpServer, Responder};
use aws_handler::AwsHandler;
use std::sync::Arc;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

#[get("/health")]
async fn health() -> impl Responder {
    "I'm alive!"
}

async fn list_buckets(aws_handler: web::Data<Arc<AwsHandler>>) -> impl Responder {
    match aws_handler.list_buckets().await {
        Ok(_) => "Buckets listed successfully",
        Err(_) => "Failed to list buckets",
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let profile = "KyribaTeamDevOpsEnablement-667083570110";
    let aws_handler = Arc::new(AwsHandler::new(profile).await.expect("Failed to create AWS handler"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(aws_handler.clone()))
            .service(hello)
            .service(health)
            .route("/list_buckets", web::get().to(list_buckets))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}