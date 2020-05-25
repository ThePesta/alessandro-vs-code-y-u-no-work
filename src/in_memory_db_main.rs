use std::collections::HashMap;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{Serialize};
use uuid::Uuid;
use island::Island;
use std::sync::Arc;
use tokio::sync::Mutex;

mod island;

#[derive(Clone, Debug)]
struct IslandsStore {
    data: HashMap<Uuid, Island>
}

async fn post_island_request_handler(command: web::Json<island::CreateIsland>, db: web::Data<Arc<Mutex<IslandsStore>>>) -> impl Responder {
    #[derive(Serialize)]
    struct Response {
        id: Uuid
    }

    let store = db.into_inner();
    
    match Island::new(command.into_inner()) {
        Ok(island) => {
            let mut store = store.lock().await;
            store.data.insert(island.id.clone(), island.clone());

            println!("{:?}", store);
            HttpResponse::Ok().json(Response{ id: island.id })
            // Lock guard dropped here and the lock is released
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let data = Arc::new(Mutex::new(IslandsStore { data: HashMap::new() }));
    
    // by doing data.clone() we're cloning a pointer
    HttpServer::new(move || App::new().wrap(Logger::default()).route("/islands", web::post().to(post_island_request_handler)).data(data.clone()))
        .bind("0.0.0.0:9999")?
        .workers(2)
        .run()
        .await
}
