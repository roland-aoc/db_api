
// #[macro_export]
// macro_rules! model_comparer {
//     ( $data_set:expr, $get_schema:expr ) => {{
//         // let mut temp_vec = Vec::new();
//         // $(
//         //     // temp_vec.push($x);
//         //     // ($x as DataSet).length
//         //     $x{}
//         // )*

//         let value = serde_json::to_value($data_set).unwrap();
//         if let Value::Object(map) = value {
//             let user_map: HashMap<String, Value> = map.into_iter().collect();
//             println!("User HashMap: {:?}", user_map);
//             $get_schema;
//         } else {
//             panic!("Serialized value was not an object");
//         }
//         // temp_vec
//     }};
// }

// #[macro_export]
// macro_rules! iterative_macro {
//     ( $data_set:expr, $get_schema:expr ) => {{}};
// }

// #[allow(dead_code)]
// fn macro_debug(data_set: DataSet, connection: &mut Connection) {
//     let value = serde_json::to_value(&data_set).unwrap();
//     if let Value::Object(map) = value {
//         let data_set_map: HashMap<String, Value> = map.into_iter().collect();
//         println!("User HashMap: {:?}", data_set_map);
//         let form = String::from("form");
//         let db_schema = connection.get_schema(&form);
//         //TODO handle optional
//         compare_states(&db_schema.unwrap(), &data_set_map);
//     } else {
//         panic!("Serialized value was not an object");
//     }
// }

pub trait Migrate {
    fn migrate(&self);
}