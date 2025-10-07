use serde_json;

#[derive(Debug)]
pub struct Boat {
    submitDate: String,
    submitSource: String
}

#[derive(Debug, PartialEq)]
pub struct DataSet {
    // pub boat: Boat,
    pub email: String,
}