use serde_json;

#[derive(serde::Deserialize)]
pub struct Boat {
    pub submitDate: String,
    pub submitSource: String
}
