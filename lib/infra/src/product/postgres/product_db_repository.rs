use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

use domain::product::Product;
use domain::product::ProductRepository;

use crate::postgres::db_connection::DbConnection;
use crate::product::postgres::{
    product_db_mapper::ProductDbMapper,
    product_db_dto::ProductDbDto,
    product_schema::products::dsl::*,
};

pub struct ProductDbRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl ProductRepository for ProductDbRepository {
    async fn find_by_id(&self, dog_fact_id: i32) -> Result<SomeEntity, Box<dyn Error>> {
        let conn =
            self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let result = dog_facts.filter(id.eq(dog_fact_id)).get_result::<DogFact>(&conn);

        match result {
            Ok(model) => Ok(DogFactDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<SomeEntity>, Box<dyn Error>> {
        let conn =
            self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let results = dog_facts.load::<DogFact>(&conn);

        match results {
            Ok(models) => Ok(models
                .into_iter()
                .map(DogFactDbMapper::to_entity)
                .collect::<Vec<SomeEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
