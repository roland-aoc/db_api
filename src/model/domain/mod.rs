use serde_json;

pub struct Boat {
    submitDate: String,
    submitSource: String
}

pub struct DataSet {
    boat: Boat,
}