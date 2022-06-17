use crate::systemcore::systemcore::create_storage_driver;
use std::env;
extern crate dotenv;

pub async fn connect_storage() -> darkbird::Storage<std::string::String, std::string::String> {
    dotenv::dotenv().expect("Failed to read .env file");

    let storage_name = env::var("STORAGE_NAME").expect("STORAGE_NAME not found");
    create_storage_driver(&storage_name).await
}
