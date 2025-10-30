use crate::infrastructure::database::DatabaseError;
use crate::infrastructure::database::{self, ActiveConnection, Field};
use crate::model::domain::DataSet;

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
    persist(data_set)?;
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
fn persist(data_set: DataSet) -> Result<(), RepositoryError> {
    // :Result<Connection, Box<dyn std::error::Error>>

    match database::connect() {
        Ok(mut connection) => {
            let fields = build_insertion_fields(data_set);
            match connection.insert(fields, "form") {
                Ok(()) => Ok(()),
                Err(e) => Err(e)
                    .map_err(|e| RepositoryErrorKind::Inner(e))
                    .map_err(|kind| RepositoryError { kind }),
            }
        }
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
        sql_type: String::from("int"),
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
        dotenvy::dotenv().ok();
        let data_set = DataSet { length: 123 };
        persist(data_set).expect("");
    }
}
