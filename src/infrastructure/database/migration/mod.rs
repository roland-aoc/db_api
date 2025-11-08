use crate::infrastructure::database::{ActiveConnection, Connection, DatabaseError};
use crate::model::domain::DataSet;

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

        //you could also just do this at runtime
        let value = serde_json::to_value($data_set).unwrap();
        if let serde_json::Value::Object(map) = value {
            let user_map: HashMap<String, serde_json::Value> = map.into_iter().collect();
            println!("User HashMap: {:?}", user_map);
            $get_schema;
        } else {
            panic!("Serialized value was not an object");
        }
        // temp_vec
    }};
}

fn macro_debug(data_set: DataSet, connection: &mut Connection) {
    let value = serde_json::to_value(&data_set).unwrap();
    if let serde_json::Value::Object(map) = value {
        let data_set_map: HashMap<String, serde_json::Value> = map.into_iter().collect();
        println!("User HashMap: {:?}", data_set_map);
        let form = String::from("form");
        let db_schema = connection.get_schema(&form);
        //TODO handle optional
        compare_states(&db_schema, &data_set_map);
    } else {
        panic!("Serialized value was not an object");
    }
}

fn compare_states(
    schema: &HashMap<String, serde_json::Value>,
    data_set_map: &HashMap<String, serde_json::Value>,
) -> Option<HashMap<String, serde_json::Value>> {
    for line in schema.into_iter() {
        data_set_map.get(line.0);
    }
    Ok(HashMap::new())
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        infrastructure::database::{ActiveConnection, migration::schema_is_updated},
        model::domain::DataSet,
    };

    #[test]
    fn vec2_test1() {
        dotenvy::dotenv().ok();
        let d = DataSet {
            submit_source: String::from(""),
            submit_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            build_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            upload_server_url: String::from(""),
            event_observer: String::from(""),
            author_email_address: String::from(""),
            hull_material: crate::model::domain::Rumpfmaterial::STAHL.to_string(),
            rig_type: String::from(""),
            length: 0.0,
            width: 0.0,
            draft: 0.0,
            additional_notes: String::from(""),
            keel_material: String::from(""),
            displacement: 0.0,
            total_weight: 0.0,
            number_of_engines: 0,
            engine_power_per_unit: 0.0,
            engine_manufacturer: String::from(""),
            engine_models: String::from(""),
            engine_serial_numbers: String::from(""),
            engine_hours: 0,
            boat_name: String::from(""),
            hin_number: String::from(""),
            trailer_or_cradle: vec![],
            ce_certificate: vec![],
            vat_proof: vec![],
            ownership_proof: vec![],
            registration_papers: String::from(""),
            previous_operating_area: String::from(""),
            number_of_previous_owners: 0,
            lay_up_period: String::from(""),
            antifouling_system: String::from(""),
            sail_configuration: String::from(""),
            standing_rigging_age: 0,
            file_upload: vec![],
            pictures: vec![],
            time_to_submit: String::from(""),
            preview: String::from(""),
            validated_new_required_field_ids: String::from(""),
            path: String::from(""),
        };
        let mut c = crate::repository::get_connection().unwrap();
        let table_name: String = String::from("form");
        // model_comparer!(&d, c.get_schema(&table_name));
        macro_debug(d, &mut c);
        // assert_eq!(model_reader!(struct something).get(2).unwrap(), &String::from("4"));
    }
}
