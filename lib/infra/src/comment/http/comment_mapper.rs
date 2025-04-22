use domain::comment::Comment;

use crate::comment::http::comment_dto::CommentDto;

pub struct CommentMapper {}

impl CommentMapper {
    pub fn covert(dto: &CommentDto) -> Comment {
        Comment { id: dto.id, name: dto.name }
    }
}
