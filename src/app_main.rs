use std::collections::HashMap;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use island::{Island, IslandRepository};
use diesel::RunQueryDsl;

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

    // use schema::islands;
    // use diesel::dsl::{insert_into};

    // #[derive(Deserialize, Insertable)]
    // #[table_name = "islands"]
    // struct NewIslandRow {
    //     id: Uuid,
    //     owner_id: Uuid,
    //     name: String,
    //     is_active: bool,
    // }

    match Island::new(command.into_inner()) {
        Ok(island) => {
            // let island_row = NewIslandRow {
            //     id: island.id,
            //     owner_id: island.owner_id,
            //     name: island.name.into(),
            //     is_active: island.is_active,
            // };

            // use crate::schema::islands::dsl::{islands as islands_dsl};
            // insert_into(islands_dsl).values(&island_row).execute(&connection.get().unwrap()).unwrap();
            match island_repository.save(island) {
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

    // use diesel::prelude::*;

    // let connection = establish_connection_pool();

    // {
    //     use crate::schema::islands::dsl::{islands, id, owner_id};
    //     let _results = islands
    //         .select((id, owner_id))
    //         .limit(5)
    //         .load::<IslandDb>(&connection.get().unwrap())
    //         .expect("Error loading islands");
    // }

    HttpServer::new(move ||
            App::new()
                .wrap(Logger::default())
                .route("/islands", web::post().to(post_island_request_handler))
                // .data(connection.clone())
        )
        .bind("0.0.0.0:9998")?
        .workers(2)
        .run()
        .await

}
