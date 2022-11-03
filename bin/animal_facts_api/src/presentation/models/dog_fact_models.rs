use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DogFactPayload {
    pub fact_id: i32,
    pub txt: String,
}
