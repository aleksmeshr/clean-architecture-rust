use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductPayload {
    pub id: i32,
    pub name: String,
}
