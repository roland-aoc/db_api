use database::{self, ActiveConnection};
use database::domain::{Field};
use database::{Connection, DatabaseError};
use crate::model::entity::DataSet;

#[derive(Debug)]
pub struct RepositoryError {
    pub kind: RepositoryErrorKind,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RepositoryErrorKind {
    Inner(DatabaseError),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error in repository `xxx`")
    }
}

impl std::error::Error for RepositoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            RepositoryErrorKind::Inner(e) => Some(e),
        }
    }
}

// impl From<DatabaseError> for RepositoryError {
//     fn from(err: DatabaseError) -> Self {
//         RepositoryError::Repo(err)
//     }
// }

pub fn create(data_set: DataSet) -> Result<(), RepositoryError> {
    let connection = get_connection()?;

    // let _ = migration::schema_is_updated(&mut connection, &data_set);
    // migration::execute(&data_set);
    
    let fields = build_insertion_fields(data_set);
    persist(&fields, connection).ok();
    Ok(())
    // .map_err(|_| RepositoryErrorKind::Default).map_err(|kind | RepositoryError {kind})
    // Ok(())
    // match persist(data_set) {
    //     Ok(_p) => Ok(()),
    //     Err(_e) => , //TODO create traceable error
    // }
}

// pub fn read() -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }

// pub fn update() -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }

// pub fn delete() -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }

// fn persist(data_set: DataSet) -> Result<(), Box<dyn std::error::Error>> {
pub fn get_connection() -> Result<Connection, RepositoryError> {
    // :Result<Connection, Box<dyn std::error::Error>>

    match database::connect() {
        Ok(connection) => Ok(connection),
        Err(e) => Err(e)
            .map_err(|e| RepositoryErrorKind::Inner(e))
            .map_err(|kind| RepositoryError { kind }),
    }
}

fn persist(
    fields: &Vec<Field>,
    mut connection: database::Connection,
) -> Result<(), RepositoryError> {
    match connection.insert(fields, "form") {
        Ok(()) => Ok(()),
        Err(e) => Err(e)
            .map_err(|e| RepositoryErrorKind::Inner(e))
            .map_err(|kind| RepositoryError { kind }),
    }
}

// database::connect().map(|mut connection| {
//         let fields = build_insertion_fields(data_set);
//         match connection.insert(fields, "form") {
//             Ok(()) => Ok(()),
//             Err(e) => Err(e)
//         }}
//     ).map_err(|e| RepositoryErrorKind::Inner(e))
//     .map_err(|kind| RepositoryError {kind});
//     Ok(())

// match database::connect() {
//     Ok(mut connection) => {
//         let rows = build_insertion_rows(data_set);
//         match connection.insert(rows, "form") {
//             Ok(()) => Ok(()),
//             Err(_e) => panic!("persist panics insertion didn't go well")
//         }
//         // Ok(())
//     },
//     Err(e) => {}
// {
//     let DatabaseError(dbe) = e;
//     let s = dbe.to_string();
//     Err(s.into())},

fn build_insertion_fields(_data_set: DataSet) -> Vec<Field> {
    let v = vec![Field {
        name: String::from("length"),
        type_str: String::from("int"),
    }];
    // for data_set
    v
}

// fn persist(data_set: DataSet) -> Result<(), jdbc::errors::Error> {
//     let datasource = jdbc::Builder::new()
//     .jdbc_url(&connection_str)
//     .build()
//     .expect("Failed to initialize data source.");

//     let conn = datasource.get_connection()?;

//     let statement = conn
//     .prepare_statement("select username,age from user where id=?")?
//         .set_int(1, 1000)?;

//     let result = statement.execute_query()?;

//     while result.next()? {
//         let username = result.get_string(1)?;
//         let age = result.get_int(1)?;
//         println!("user:{:?} age:{:?}", username, age);
//     }
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persist_test1() {
        let data_set = DataSet { submit_source: String::from(""), submit_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), build_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), upload_server_url: String::from(""), event_observer: String::from(""), author_email_address: String::from(""), hull_material: crate::model::entity::Rumpfmaterial::STAHL.to_string(), rig_type: String::from(""), length: 0.0, width: 0.0, draft: 0.0, additional_notes: String::from(""), keel_material: String::from(""), displacement: 0.0, total_weight: 0.0, number_of_engines: 0, engine_power_per_unit: 0.0, engine_manufacturer: String::from(""), engine_models: String::from(""), engine_serial_numbers: String::from(""), engine_hours: 0, boat_name: String::from(""), hin_number: String::from(""), trailer_or_cradle: vec![], ce_certificate: vec![], vat_proof: vec![], ownership_proof: vec![], registration_papers: String::from(""), previous_operating_area: String::from(""), number_of_previous_owners: 0, lay_up_period: String::from(""), antifouling_system: String::from(""), sail_configuration: String::from(""), standing_rigging_age: 0, file_upload: vec![], pictures: vec![], time_to_submit: String::from(""), preview: String::from(""), validated_new_required_field_ids: String::from(""), path: String::from(""), };
        let connection = get_connection().unwrap();
        let fields = build_insertion_fields(data_set);
        persist(&fields, connection).expect("");
    }
}
