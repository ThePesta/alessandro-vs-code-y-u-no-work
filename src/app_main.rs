use std::collections::HashMap;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::Serialize;
use uuid::Uuid;
use island_repository::IslandRepositoryPostgres;
use island::{Island, IslandRepository};

#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;

mod island;
mod schema;
mod island_repository;

type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> Pool {
    let database_url = "postgres://postgres:test@localhost:5432/not-mongodb";

    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    pool
}

#[derive(Clone, Debug)]
struct IslandsStore {
    data: HashMap<Uuid, Island>
}

async fn post_island_request_handler<R: IslandRepository>(command: web::Json<island::CreateIsland>, island_repository: web::Data<R>) -> impl Responder {

    #[derive(Serialize)]
    struct Response {
        id: Uuid
    }

    match Island::new(command.into_inner()) {
        Ok(island) => {
            match island_repository.save(&island) {
                Ok(()) => HttpResponse::Ok().json(Response{ id: island.id }),
                Err(_) => HttpResponse::InternalServerError().finish()
            }
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[derive(Queryable)]
pub struct IslandDb {
    pub id: Uuid,
    pub owner_id: Uuid,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let connection_pool = establish_connection_pool();
    let repository = IslandRepositoryPostgres {
        connection_pool
    };

    HttpServer::new(move ||
            App::new()
                .wrap(Logger::default())
                .route("/islands", web::post().to(post_island_request_handler::<IslandRepositoryPostgres>))
                .data(repository.clone())
        )
        .bind("0.0.0.0:9998")?
        .workers(2)
        .run()
        .await
}
