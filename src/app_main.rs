use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};
use uuid::Uuid;
use island::Island;

mod island;

async fn post_island_request_handler(command: web::Json<island::CreateIsland>) -> impl Responder {
    #[derive(Serialize)]
    struct Response {
        id: Uuid
    }

    match Island::new(command.into_inner()) {
        Ok(Island { id, .. }) => HttpResponse::Ok().json(Response{ id }),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/islands", web::post().to(post_island_request_handler)))
        .bind("0.0.0.0:8088")?
        .run()
        .await
}
