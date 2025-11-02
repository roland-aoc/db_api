use crate::infrastructure::database::{ActiveConnection, Connection, DatabaseError};
use crate::model::domain::DataSet;

#[macro_export]
macro_rules! model_reader {
    ( $x:expr, $func:expr ) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //     // temp_vec.push($x);
        //     // ($x as DataSet).length
        //     $x{}
        // )*
        let value = serde_json::to_value($x).unwrap();
        if let serde_json::Value::Object(map) = value {
            let user_map: std::collections::HashMap<String, serde_json::Value> =
                map.into_iter().collect();
            println!("User HashMap: {:?}", user_map);
            $func;
        } else {
            panic!("Serialized value was not an object");
        }
        // temp_vec
    }};
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
    // use super::*;

    use crate::{infrastructure::database::migration::schema_is_updated, model::domain::DataSet};

    #[test]
    fn vec2_test1() {
        dotenvy::dotenv().ok();
        let d = DataSet { length: 2 };
        let mut c = crate::repository::get_connection().unwrap();
        model_reader!(&d, schema_is_updated(&mut c, &d));
        // assert_eq!(model_reader!(struct something).get(2).unwrap(), &String::from("4"));
    }
}
