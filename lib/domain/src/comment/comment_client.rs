use async_trait::async_trait;
use mockall::automock;
use std::error::Error;

use crate::comment::comment::Comment;

#[automock]
#[async_trait(?Send)]
pub trait CommentClient {
    async fn find_by_id(&self, id: i32) -> Result<Comment, Box<dyn Error>>;
}
