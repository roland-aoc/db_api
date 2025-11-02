// use crate::model::domain::DataSet;
// use crate::model::http::Payload;

// impl From<Payload> for DataSet {
//     fn from(payload: Payload) -> Self {
//         Self {
//             submit_source: String::from(""),
//             submit_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
//             build_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
//             upload_server_url: String::from(""),
//             event_observer: String::from(""),

//             author_email_address: String::from(""),
//             hull_material: match payload.q32_rumpfmaterial {
//                 crate::model::http::Rumpfmaterial::STAHL => String::from("Stahl"),
//                 crate::model::http::Rumpfmaterial::GFK => String::from("GFK"),
//                 crate::model::http::Rumpfmaterial::ALUMINIUM => String::from("Aluminium"),
//                 crate::model::http::Rumpfmaterial::SONSTIGE(other) => other,
//             },
//             rig_type: payload.q4_rigg_typ,
//             length: payload.q5_laenge.parse::<f32>().unwrap_or(0.0),
//             width: 0.0,
//             draft: 0.0,
//             additional_notes: String::from(""),
//             keel_material: String::from(""),
//             displacement: 0.0,
//             total_weight: 0.0,
//             number_of_engines: 0,
//             engine_power_per_unit: 0.0,
//             engine_manufacturer: String::from(""),
//             engine_models: String::from(""),
//             engine_serial_numbers: String::from(""),
//             engine_hours: 0,
//             boat_name: String::from(""),
//             hin_number: String::from(""),
//             trailer_or_cradle: vec![],
//             ce_certificate: vec![],
//             vat_proof: vec![],
//             ownership_proof: vec![],
//             registration_papers: String::from(""),
//             previous_operating_area: String::from(""),
//             number_of_previous_owners: 0,
//             lay_up_period: String::from(""),
//             antifouling_system: String::from(""),
//             sail_configuration: String::from(""),
//             standing_rigging_age: 0,
//             file_upload: vec![],

//             pictures: vec![],

//             time_to_submit: String::from(""),
//             preview: String::from(""),
//             validated_new_required_field_ids: String::from(""),
//             path: String::from(""),
//         }
//         // Self {
//         //     length: payload.q5_laenge.parse::<f32>().unwrap(),
//         //     additional_notes: payload.q5_laenge.parse::<String>().unwrap()
//         // }
//     }
// }

// pub fn to_data_set(payload: Payload) -> DataSet {
//     return payload.into();
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn to_model_test1() {
//         let _input = Payload { 
//             q5_laenge: String::from("123"), 
//             q32_rumpfmaterial: crate::model::http::Rumpfmaterial::ALUMINIUM, 
//             q4_rigg_typ: String::from("") 
//         };
//     }
// }