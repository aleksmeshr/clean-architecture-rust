use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

use crate::postgres::{
    db_connection::DbConnection, mappers::DogFactDbMapper, models::DogFact,
    schema::dog_facts::dsl::*,
};
use animal_facts_ports::dog_facts::DogFacts;
use animal_facts_domain::dog_fact_entity::DogFactEntity;

pub struct DogFactsRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl DogFacts for DogFactsRepository {
    async fn get_dog_fact_by_id(&self, dog_fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>> {
        let conn =
            self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let result = dog_facts.filter(id.eq(dog_fact_id)).get_result::<DogFact>(&conn);

        match result {
            Ok(model) => Ok(DogFactDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>> {
        let conn =
            self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let results = dog_facts.load::<DogFact>(&conn);

        match results {
            Ok(models) => Ok(models
                .into_iter()
                .map(DogFactDbMapper::to_entity)
                .collect::<Vec<DogFactEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
