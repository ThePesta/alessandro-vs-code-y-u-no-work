use std::collections::HashMap;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{Serialize};
use uuid::Uuid;
use island::Island;
use std::sync::Arc;
use tokio::sync::Mutex;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

mod island;
mod schema;

pub fn establish_connection() -> PgConnection {
  let database_url = "postgres://postgres:test@localhost:5432/not-mongodb";

  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

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

#[derive(Queryable)]
pub struct IslandDb {
    pub id: Uuid,
    pub owner_id: Uuid,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let data = Arc::new(Mutex::new(IslandsStore { data: HashMap::new() }));

    use schema::islands;
    use serde::Deserialize;
    use diesel::prelude::*;
    use uuid::Uuid;
    use diesel::dsl::{insert_into};

    let connection = establish_connection();

    #[derive(Deserialize, Insertable)]
    #[table_name = "islands"]
    struct NewIslandRow {
        id: Uuid,
        owner_id: Uuid,
        name: String,
        is_active: bool,
    }

    let island_row = NewIslandRow {
        id: Uuid::new_v4(),
        owner_id: Uuid::new_v4(),
        name: "Yoshi's Island".into(),
        is_active: true,
    };
    
    use crate::schema::islands::dsl::{islands as islands_dsl};
    insert_into(islands_dsl).values(&island_row).execute(&connection).unwrap();

    {
        use crate::schema::islands::dsl::{islands, id, owner_id};
        let results = islands
            .select((id, owner_id))
            .limit(5)
            .load::<IslandDb>(&connection)
            .expect("Error loading islands");
    }
    
    // by doing data.clone() we're cloning a pointer
    HttpServer::new(move || App::new().wrap(Logger::default()).route("/islands", web::post().to(post_island_request_handler)).data(data.clone()))
        .bind("0.0.0.0:9999")?
        .workers(2)
        .run()
        .await
}
 