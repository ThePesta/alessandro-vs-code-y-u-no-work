use std::collections::HashMap;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};
use uuid::Uuid;
use island::Island;
use std::sync::{Arc, Mutex};

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
            match store.try_lock() {
                Ok(mut store) => {
                    println!("{:?}", store);
                    store.data.insert(island.id.clone(), island.clone());
                    HttpResponse::Ok().json(Response{ id: island.id })
                }
                Err(err) => {
                    eprintln!("Something went awfully wrong: {:?}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = Arc::new(Mutex::new(IslandsStore { data: HashMap::new() }));
    
    HttpServer::new(move || App::new().route("/islands", web::post().to(post_island_request_handler)).data(data.clone()))
        .bind("0.0.0.0:8088")?
        .run()
        .await
}
