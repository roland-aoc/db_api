use crate::model::{entity};
use crate::repository::{self, RepositoryError};
use crate::util;

#[derive(Debug)]
pub struct ServiceError {
    kind: ServiceErrorKind,
}

impl std::error::Error for ServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ServiceErrorKind::Inner(e) => Some(e),
        }
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error in service `xxx`")
    }
}

#[derive(Debug)]
pub enum ServiceErrorKind {
    Inner(RepositoryError),
}

pub fn handle(data_set: entity::DataSet) -> Result<(), ServiceError> {
    // let model: Option<domain::DataSet> = Some(mapper::to_model::<domain::DataSet>(payload));
    // let model = mapper::to_data_set(payload);
    let _len = validate(data_set.length)?;

    // repository::persist(model.expect("Model is empty!"))
    repository::create(data_set).map_err(|e| ServiceErrorKind::Inner(e))
    .map_err(|kind| ServiceError {kind})
}

//TODO this has to make sense
fn validate(input: f32) -> Result<usize, ServiceError> {
    println!("Validate len, which is {input}");
    Ok(util::check_len(""))
}

#[allow(dead_code)]
fn email(_data: &str) -> &str {
    "writerolandkurmann@gmail.com"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_test1() {
        assert_eq!(
            email(
                "{ \"id\": null, \"name\": \"Sea Breeze\", \"boat_type\": \"Sail\", \"length\": 12.5, \"year\": 2018 }"
            ),
            "writerolandkurmann@gmail.com"
        );
    }
}