use crate::infrastructure::database::{self, ActiveConnection, Row};
use crate::infrastructure::database::DatabaseError;
use crate::model::domain::DataSet;

pub fn create(data_set: DataSet) -> Result<(), Box<dyn std::error::Error>> {
    match persist(data_set) {
        Ok(_p) => Ok(()),
        Err(_e) => Err(Box::from("Create didn't work")), //TODO create traceable error
    }
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

fn persist(data_set: DataSet) -> Result<(), Box<dyn std::error::Error>> {
    // :Result<Connection, Box<dyn std::error::Error>>
    match database::connect() {
        Ok(mut connection) => {
            let rows = build_insertion_rows(data_set);
            match connection.insert(rows, "form") {
                Ok(()) => Ok(()),
                Err(_e) => panic!("persist panics insertion didn't go well")
            }
            // Ok(())
        },
        Err(e) => {
            let DatabaseError(dbe) = e;
            let s = dbe.to_string();
            Err(s.into())},
    }
}

fn build_insertion_rows(_data_set: DataSet) -> Vec<Row> {
    let v = vec!(Row {label: String::from("length"), sql_type: String::from("int")});
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
        let data_set = DataSet {
            length: 123,
        };
        persist(data_set).expect("");
    }
}
