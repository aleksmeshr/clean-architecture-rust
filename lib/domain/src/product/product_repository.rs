use async_trait::async_trait;
use mockall::automock;
use std::error::Error;

use crate::product::product::Product;

#[automock]
#[async_trait(?Send)]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Product, Box<dyn Error>>;
}
