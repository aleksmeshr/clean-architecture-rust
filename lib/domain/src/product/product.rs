#[derive(Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
}

impl Product {
    pub fn new(id: i32, name: String) -> Self {
        Product { id, name }
    }
}
