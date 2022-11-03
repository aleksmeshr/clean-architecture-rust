use async_trait::async_trait;

use animal_facts_domain::cat_fact_entity::CatFactEntity;

use mockall::automock;
use std::error::Error;

#[automock]
#[async_trait(?Send)]
pub trait CatFacts {
    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, Box<dyn Error>>;
    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>>;
}
