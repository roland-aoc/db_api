use std::error::Error;

use crate::infrastructure::database::connection::{self};
use crate::infrastructure::database::error::DatabaseError;
use crate::model::domain::DataSet;

pub fn create(data_set: DataSet) -> Result<(), Box<dyn std::error::Error>> {
    match persist(data_set) {
        Ok(p) => panic!("Works"),
        Err(E) => panic!("doesn't work"),
    }
    Ok(())
}

pub fn read() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn delete() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn persist(data_set: DataSet) -> Result<(), Box<dyn std::error::Error>> {
    // let greeting_file = File::open("hello.txt")?;
    // Ok(())
    match connection::connect() {
        Ok(c) => Ok(()),
        Err(e) => {
            let DatabaseError(dbe) = e;
            let s = dbe.to_string();
            Err(s.into())},
    }
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
        let data_set = DataSet {
            email: String::from(""),
        };
        persist(data_set).expect("");
    }
}
