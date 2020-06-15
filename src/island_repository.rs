use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::island::{IslandRepository, Island};
use crate::Pool;
use crate::schema;

use schema::islands;
use diesel::dsl::{insert_into};
use schema::islands::dsl::{islands as islands_dsl};

struct IslandRepositoryPostgres {
    connection_pool: Pool
}

impl IslandRepository for IslandRepositoryPostgres {
    fn save(&self, island: Island) {

      #[derive(Deserialize, Insertable)]
      #[table_name = "islands"]
      struct NewIslandRow {
          id: Uuid,
          owner_id: Uuid,
          name: String,
          is_active: bool,
      }

      let island_row = NewIslandRow {
          id: island.id,
          owner_id: island.owner_id,
          name: island.name.into(),
          is_active: island.is_active,
      };

      insert_into(islands_dsl).values(&island_row).execute(&self.connection_pool.get().unwrap()).unwrap();
    }
}
