use crate::systemcore::systemmessages::{ErrorsMessages, SuccessMessages};

use darkbird::{Options, Storage, StorageType};

async fn create_or_open_storage(
    path: &str,
    storage_name: &str,
    total_size: usize,
) -> Storage<std::string::String, std::string::String> {
    type Pid = std::string::String;
    Storage::<Pid, std::string::String>::open(Options::new(
        path,
        storage_name,
        total_size,
        StorageType::DiskCopies,
    ))
    .await
    .unwrap()
}

pub async fn create_storage_driver(
    storage_name: &str,
) -> Storage<std::string::String, std::string::String> {
    let path: &str = "./.appData";
    let total_size: usize = 1000;

    create_or_open_storage(path, storage_name, total_size).await
}

pub async fn insert(
    key: std::string::String,
    value: std::string::String,
    storage_driver: &Storage<std::string::String, std::string::String>,
) -> Result<SuccessMessages, ErrorsMessages> {
    match storage_driver
        .insert(std::string::String::from(key), value)
        .await
    {
        Ok(_) => Ok(SuccessMessages::SuccesOnInsert),
        Err(_) => Err(ErrorsMessages::ErrorOnInsert),
    }
}
pub async fn update(
    id: &str,
    value: std::string::String,
    storage_driver: &Storage<std::string::String, std::string::String>,
) -> Result<SuccessMessages, ErrorsMessages> {
    match storage_driver
        .insert(std::string::String::from(id), value)
        .await
    {
        Ok(_) => Ok(SuccessMessages::SuccesOnUpdate),
        Err(_) => Err(ErrorsMessages::ErrorOnUpdate),
    }
}

pub fn get_all(
    storage_driver: &Storage<std::string::String, std::string::String>,
) -> dashmap::iter::Iter<'_, std::string::String, std::string::String> {
    storage_driver.iter()
}

pub fn get_one(
    id: &str,
    storage_driver: &Storage<std::string::String, std::string::String>,
) -> Option<std::string::String> {
    storage_driver.lookup(&std::string::String::from(id))
}

pub async fn remove(
    id: &str,
    storage_driver: &Storage<std::string::String, std::string::String>,
) -> Result<SuccessMessages, ErrorsMessages> {
    match storage_driver.remove(std::string::String::from(id)).await {
        Ok(_) => Ok(SuccessMessages::SuccesOnDelete),
        Err(_) => Err(ErrorsMessages::ErrorOnDelete),
    }
}
