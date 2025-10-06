use crate::{repository};
use crate::model::{http, domain, mapper};

pub fn handle(payload: http::Payload) -> Result<(), Box<dyn std::error::Error>>{
    let model: Option<domain::DataSet> = Some(mapper::to_model::<domain::DataSet>(payload));
    repository::persist(model.expect("Model is empty!"))
}