use domain::comment::comment::Comment;

use crate::comment::http::comment_dto::CommentDto;

pub struct CommentMapper {}

impl CommentMapper {
    pub fn to_entity(dto: CommentDto) -> Comment {
        Comment { id: dto.id, text: dto.text }
    }
}
