use uuid::Uuid;
use serde::Deserialize;

use crate::island::{IslandRepository, Island};
use crate::Pool;
use crate::schema;

use schema::islands;
use diesel::RunQueryDsl;
use diesel::dsl::{insert_into};
use schema::islands::dsl::{islands as islands_dsl};

#[derive(Clone)]
pub struct IslandRepositoryPostgres {
    pub connection_pool: Pool
}

impl IslandRepository for IslandRepositoryPostgres {
    fn save(&self, island: &Island) -> Result<(), Box<dyn std::error::Error>> {

      #[derive(Deserialize, Insertable)]
      #[table_name = "islands"]
      struct NewIslandRow<'a> {
          id: Uuid,
          owner_id: Uuid,
          name: &'a str,
          is_active: bool,
      }

      let island_row = NewIslandRow {
          id: island.id,
          owner_id: island.owner_id,
          name: island.name.as_ref(),
          is_active: island.is_active,
      };

      let connection = self.connection_pool.get()?;
      insert_into(islands_dsl).values(&island_row).execute(&connection)?;
      Ok(())
    }
}
