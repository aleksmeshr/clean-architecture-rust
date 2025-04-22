use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentDto {
    pub id: i32,
    pub text: String,
}
