use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

use domain::product::{product::Product, product_repository::ProductRepository};

use super::{
    product_db_dto::ProductDbDto, product_db_mapper::ProductDbMapper, product_schema::products::dsl,
};
use crate::postgres::db_connection::DbConnection;

pub struct ProductDbRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl ProductRepository for ProductDbRepository {
    async fn find_by_id(&self, product_id: i32) -> Result<Product, Box<dyn Error>> {
        let mut conn =
            self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let result =
            dsl::products.filter(dsl::id.eq(product_id)).get_result::<ProductDbDto>(&mut conn);

        match result {
            Ok(model) => Ok(ProductDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
