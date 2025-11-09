use crate::infrastructure::database::{ActiveConnection, Connection, DatabaseError};
use crate::model::domain::DataSet;

use serde_json::Value;
use std::collections::HashMap;

#[macro_export]
macro_rules! model_comparer {
    ( $data_set:expr, $get_schema:expr ) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //     // temp_vec.push($x);
        //     // ($x as DataSet).length
        //     $x{}
        // )*

        let value = serde_json::to_value($data_set).unwrap();
        if let Value::Object(map) = value {
            let user_map: HashMap<String, Value> = map.into_iter().collect();
            println!("User HashMap: {:?}", user_map);
            $get_schema;
        } else {
            panic!("Serialized value was not an object");
        }
        // temp_vec
    }};
}

#[macro_export]
macro_rules! iterative_macro {
    ( $data_set:expr, $get_schema:expr ) => {{}};
}

#[allow(dead_code)]
fn macro_debug(data_set: DataSet, connection: &mut Connection) {
    let value = serde_json::to_value(&data_set).unwrap();
    if let Value::Object(map) = value {
        let data_set_map: HashMap<String, Value> = map.into_iter().collect();
        println!("User HashMap: {:?}", data_set_map);
        let form = String::from("form");
        let db_schema = connection.get_schema(&form);
        //TODO handle optional
        compare_states(&db_schema.unwrap(), &data_set_map);
    } else {
        panic!("Serialized value was not an object");
    }
}

fn compare_states(
    _schema: &HashMap<String, Value>,
    _data_set_map: &HashMap<String, Value>,
) -> HashMap<String, Value> {
    todo!()
}

pub fn schema_is_updated(
    connection: &mut Connection,
    _data_set: &DataSet,
) -> Result<bool, DatabaseError> {
    for _field in connection.get_schema("account")? {
        // Ok(false)
    }
    Ok(true)
}

pub fn execute(_data_set: &DataSet) {}

#[allow(dead_code)]
pub trait Migrate {
    fn migrate(&self);
}


#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    use crate::{
        // infrastructure::database::{ActiveConnection, migration::schema_is_updated},
        model::domain::DataSet,
    };

    #[test]
    #[ignore]
    fn model_comparer_test1() {
        dotenvy::dotenv().ok();
        let d = DataSet { submit_source: String::from(""), submit_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), build_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), upload_server_url: String::from(""), event_observer: String::from(""), author_email_address: String::from(""), hull_material: crate::model::domain::Rumpfmaterial::STAHL.to_string(), rig_type: String::from(""), length: 0.0, width: 0.0, draft: 0.0, additional_notes: String::from(""), keel_material: String::from(""), displacement: 0.0, total_weight: 0.0, number_of_engines: 0, engine_power_per_unit: 0.0, engine_manufacturer: String::from(""), engine_models: String::from(""), engine_serial_numbers: String::from(""), engine_hours: 0, boat_name: String::from(""), hin_number: String::from(""), trailer_or_cradle: vec![], ce_certificate: vec![], vat_proof: vec![], ownership_proof: vec![], registration_papers: String::from(""), previous_operating_area: String::from(""), number_of_previous_owners: 0, lay_up_period: String::from(""), antifouling_system: String::from(""), sail_configuration: String::from(""), standing_rigging_age: 0, file_upload: vec![], pictures: vec![], time_to_submit: String::from(""), preview: String::from(""), validated_new_required_field_ids: String::from(""), path: String::from(""), }; let mut c = crate::repository::get_connection().unwrap();
        let _table_name: String = String::from("form");
        // model_comparer!(&d, c.get_schema(&table_name));
        macro_debug(d, &mut c);
        // assert_eq!(model_reader!(struct something).get(2).unwrap(), &String::from("4"));
    }

    #[test]
    fn compare_states_test1() {
        dotenvy::dotenv().ok();

        let mut s = HashMap::new();
        s.insert(String::from("length"), json!(1.2345));
        s.insert(String::from("author_email_address"), json!("me@mail.com"));

        let mut d = HashMap::new();
        d.insert(String::from("width"), json!(5.4321));
        d.insert(String::from("author_email_address"), json!("me@mail.com"));

        let mut r: HashMap<String, Value> = HashMap::new();
        r.insert(String::from("width"), json!(1.2345));

        // assert_eq!(compare_states(&s, &d).unwrap(), r);
    }

    #[test]
    fn compare_migration_macro_test1() {
        let d = DataSet { submit_source: String::from(""), submit_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), build_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), upload_server_url: String::from(""), event_observer: String::from(""), author_email_address: String::from(""), hull_material: crate::model::domain::Rumpfmaterial::STAHL.to_string(), rig_type: String::from(""), length: 0.0, width: 0.0, draft: 0.0, additional_notes: String::from(""), keel_material: String::from(""), displacement: 0.0, total_weight: 0.0, number_of_engines: 0, engine_power_per_unit: 0.0, engine_manufacturer: String::from(""), engine_models: String::from(""), engine_serial_numbers: String::from(""), engine_hours: 0, boat_name: String::from(""), hin_number: String::from(""), trailer_or_cradle: vec![], ce_certificate: vec![], vat_proof: vec![], ownership_proof: vec![], registration_papers: String::from(""), previous_operating_area: String::from(""), number_of_previous_owners: 0, lay_up_period: String::from(""), antifouling_system: String::from(""), sail_configuration: String::from(""), standing_rigging_age: 0, file_upload: vec![], pictures: vec![], time_to_submit: String::from(""), preview: String::from(""), validated_new_required_field_ids: String::from(""), path: String::from(""), }; // d.migrate();
        //won't work if test is run from VSCode/rust-analyzer
        d.migrate()
        // migration_macro::compare_schema_fn_macro!(for i in 0..1 {});
        // migration_macro::compare_schema_fn_macro!(DataSet);
    }
}
