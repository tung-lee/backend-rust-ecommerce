use mongodb::{options::ClientOptions, Client};

use crate::error::{Error, Result};

pub struct MyDatabase {}

impl MyDatabase {
    // constructor
    // fn new() -> MyDatabase {}

    // async fn get_instance() {}
}

pub async fn connect(db_type: Option<String>) -> Result<Client> {
    let db_type = db_type.unwrap_or("mongodb".to_string());

    let connection_string = "mongodb://admin:password@localhost:27017".to_string();

    let client_options = ClientOptions::parse(connection_string)
        .await
        .map_err(|_| Error::DatabaseConnectionFailed)?;

    let client = Client::with_options(client_options).unwrap();

    Ok(client)
}
