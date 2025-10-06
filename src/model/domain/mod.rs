use serde_json;

#[derive(Debug)]
pub struct Boat {
    submitDate: String,
    submitSource: String
}

#[derive(Debug)]
pub struct DataSet {
    boat: Boat,
}