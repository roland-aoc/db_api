use crate::parser;
use crate::model::http::Form;
use crate::model::domain::Boat;

pub fn persist(form: Form) {
    let model: Option<Form> = Some(parser::to_model::<Boat>(form));
}