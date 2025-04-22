use async_trait::async_trait;
use std::error::Error;

use domain::comment::{comment::Comment, comment_client::CommentClient};

use super::{comment_dto::CommentDto, comment_mapper::CommentMapper};
use crate::http::http_connection::HttpConnection;

pub struct CommentHttpClient {
    pub http_connection: HttpConnection,
    pub endpoint: String,
}

#[async_trait(?Send)]
impl CommentClient for CommentHttpClient {
    async fn find_by_product_id(&self, id: i32) -> Result<Vec<Comment>, Box<dyn Error>> {
        let response = self
            .http_connection
            .make_client()
            .get(&format!("{}/comments?product_id={}", &self.endpoint, id))
            .send()
            .await;

        match response {
            Ok(r) => {
                let json = r.json::<Vec<CommentDto>>().await;

                match json {
                    Ok(payload) => Ok(payload.into_iter().map(CommentMapper::to_entity).collect()),
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
