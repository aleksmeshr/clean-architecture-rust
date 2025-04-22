#[derive(Debug, Clone)]
pub struct Comment {
    pub id: i32,
    pub text: String,
}

impl Comment {
    pub fn new(id: i32, text: String) -> Self {
        Comment { id, text }
    }
}
