use crate::models::DataTraffic;

use ghostdb::systemcore::systemmessages::{ErrorsMessages, SuccessMessages};

pub struct DbProvider {
    pub storage_driver: ghostdb::DarkbirdStorage,
}

impl DbProvider {
    pub async fn new() -> Self {
        Self {
            storage_driver: ghostdb::processor::processorcore::connect_storage().await,
        }
    }
    pub fn get_all(&self) -> Vec<DataTraffic> {
        let all_data = ghostdb::systemcore::systemcore::get_all(&self.storage_driver);
        let mut data_response: Vec<DataTraffic> = Vec::new();
        for data in all_data {
            data_response.push(DataTraffic {
                key: String::from(data.key()),
                value: String::from(data.value()),
            });
        }

        data_response
    }
    pub fn get_one(&self, id: &str) -> Option<String> {
        ghostdb::systemcore::systemcore::get_one(id, &self.storage_driver)
    }
    pub async fn insert(&self, data: DataTraffic) -> Result<SuccessMessages, ErrorsMessages> {
        ghostdb::systemcore::systemcore::insert(data.key, data.value, &self.storage_driver).await
    }
    pub async fn update(&self, id: &str, value: String) -> Result<SuccessMessages, ErrorsMessages> {
        ghostdb::systemcore::systemcore::update(id, value, &self.storage_driver).await
    }
    pub async fn remove(&self, id: &str) -> Result<SuccessMessages, ErrorsMessages> {
        ghostdb::systemcore::systemcore::remove(id, &self.storage_driver).await
    }
}
