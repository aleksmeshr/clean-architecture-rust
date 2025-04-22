use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductPayload {
    pub id: i32,
    pub name: String,
    pub comments: Vec<CommentResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentResponse {
    pub id: i32,
    pub text: String,
}
