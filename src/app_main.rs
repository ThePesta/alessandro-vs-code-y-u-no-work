use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

mod island {
    use uuid::Uuid;
    use std::str::FromStr;

    struct Name(String);

    impl Name {
        fn new(name: String) -> Result<Self, String> {
            match name {
                s if s.is_empty() => Err("Island name is missing".to_string()),
                s if s.chars().count() > 10 => Err("Island name is too long".to_string()),
                s => Ok(Name(s)),
            }
        }
    }

    struct Island {
        id: Uuid,
        owner_id: Uuid,
        name: Name,
        is_active: bool,
    }

    impl Island {
        fn new(command: CreateIsland) -> Result<Self, String> {
            let name = Name::new(command.name)?;
            let owner_id = Uuid::from_str(&command.owner_id).map_err(|_| "Invalid owner id")?;

            let island = Island{
                id: Uuid::new_v4(),
                owner_id,
                name,
                is_active: command.is_active,
            };
            
            Ok(island)
        }
    }

    struct CreateIsland {
        owner_id: String,
        name: String,
        is_active: bool,
    }

    #[cfg(test)]
    mod island_tests {
        use super::*;

        #[test]
        fn create_island_invalid_owner_id_() {
            let test_case = CreateIsland{
                // owner_id: Uuid::new_v4().to_string(),
                owner_id: "this is not a uuid".into(),
                name: "ok".into(),
                is_active: true,
            };
            
            assert!(Island::new(test_case).is_err());
        }

        #[test]
        fn create_island_invalid_name() {
            let test_case = CreateIsland{
                owner_id: Uuid::new_v4().to_string(),
                name: "".into(),
                is_active: true,
            };
            
            assert!(Island::new(test_case).is_err());
        }

        #[test]
        fn create_island_valid_command() {
            let test_case = CreateIsland{
                owner_id: Uuid::new_v4().to_string(),
                name: "valid name".into(),
                is_active: true,
            };
            
            assert!(Island::new(test_case).is_ok());
        }
    }

    #[cfg(test)]
    mod name_tests {
        use super::*;
        use proptest::prelude::*;

        #[test]
        fn weird_name() {
            let test_case = "vy꙲ꙈᴫѱΆῨῨ";

            assert!(Name::new(test_case.into()).is_ok())        
        }

        #[test]
        fn empty_string() {
            let test_case = "";

            assert!(Name::new(test_case.into()).is_err())
        }

        #[test]
        fn long_string() {
            let test_case = "12345678900";

            assert!(Name::new(test_case.into()).is_err())
        }
    }
}

async fn index(_req: HttpRequest) -> impl Responder {
  println!("{:?}", _req);

  HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
