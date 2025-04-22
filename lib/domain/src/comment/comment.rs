#[derive(Debug, Clone)]
pub struct Comment {
    pub id: i32,
    pub name: String,
}

impl Comment {
    pub fn new(id: i32, name: String) -> Self {
        Comment { id, name }
    }
}
