use async_trait::async_trait;
use std::error::Error;
use mockall::automock;

use crate::product::product::Product;

#[automock]
#[async_trait(?Send)]
pub trait ProductRepository {
    async fn find_by_id(&self, id: i32) -> Result<Product, Box<dyn Error>>;
}
