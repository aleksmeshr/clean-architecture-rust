use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CatFactPayload {
    pub fact: String,
    pub nb_chars: i32,
}
