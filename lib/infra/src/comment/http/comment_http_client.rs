use async_trait::async_trait;
use std::error::Error;

use domain::comment::Comment;

use crate::comment::http::{comment_dto::CommentDto, comment_mapper::CommentMapper};
use crate::http::http_connection::HttpConnection;

pub struct CommentHttpClient {
    pub http_connection: HttpConnection,
    pub endpoint: String,
}

#[async_trait(?Send)]
impl CommentClient for CommentHttpClient {
    async fn find_by_id(&self, id: u32) -> Result<Comment, Box<dyn Error>> {
        let response = self
            .http_connection
            .client()
            .get(&format!("{}/comment/{}", &self.endpoint, id))
            .send()
            .await;

        match response {
            Ok(r) => {
                let json = r.json::<CommentDto>().await;

                match json {
                    Ok(http_obj) => Ok(CommentMapper::to_entity(http_obj)),
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn find_all(&self) -> Result<Vec<Comment>, Box<dyn Error>> {
        let response =
            self.http_connection.client().get(&format!("{}/comment", &self.endpoint)).send().await;

        match response {
            Ok(r) => {
                let json = r.json::<CommentDto>().await;

                match json {
                    Ok(j) => Ok(j
                        .data
                        .into_iter()
                        .map(CommentMapper::to_entity)
                        .collect::<Vec<Comment>>()),
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
