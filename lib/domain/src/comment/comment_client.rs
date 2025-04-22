use async_trait::async_trait;
use mockall::automock;
use std::error::Error;

use crate::comment::comment::Comment;

#[automock]
#[async_trait(?Send)]
pub trait CommentClient: Send + Sync {
    async fn find_by_product_id(&self, id: i32) -> Result<Vec<Comment>, Box<dyn Error>>;
}
