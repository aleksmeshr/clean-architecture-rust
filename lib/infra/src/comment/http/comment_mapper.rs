use domain::comment::comment::Comment;

use super::comment_dto::CommentDto;

pub struct CommentMapper {}

impl CommentMapper {
    pub fn to_entity(dto: CommentDto) -> Comment {
        Comment { id: dto.id, text: dto.text }
    }
}
