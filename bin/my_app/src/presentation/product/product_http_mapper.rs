use crate::presentation::product::product_dto::ProductPayload;

use domain::{comment::comment::Comment, product::product::Product};

use super::product_dto::CommentResponse;

pub fn to_api(entity: Product, comments: Vec<Comment>) -> ProductPayload {
    ProductPayload {
        id: entity.id,
        name: entity.name,
        comments: comments.into_iter().map(to_comment_response).collect(),
    }
}

fn to_comment_response(comment: Comment) -> CommentResponse {
    CommentResponse { id: comment.id, text: comment.text }
}
