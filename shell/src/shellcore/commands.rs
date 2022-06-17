use ghostdb::systemcore::systemmessages::{ErrorsMessages, SuccessMessages};

pub struct Commands {
    pub storage_driver: ghostdb::DarkbirdStorage,
}

impl Commands {
    pub async fn new() -> Self {
        Self {
            storage_driver: ghostdb::processor::processorcore::connect_storage().await,
        }
    }
    pub async fn command_insert(
        &self,
        key: &str,
        value: &str,
    ) -> std::result::Result<SuccessMessages, ErrorsMessages> {
        ghostdb::systemcore::systemcore::insert(
            String::from(key),
            String::from(value),
            &self.storage_driver,
        )
        .await
    }

    pub fn command_read(&self) -> ghostdb::DbAllData {
        ghostdb::systemcore::systemcore::get_all(&self.storage_driver)
    }

    pub fn command_read_one(&self, id: &str) -> std::option::Option<std::string::String> {
        ghostdb::systemcore::systemcore::get_one(id, &self.storage_driver)
    }
    pub async fn command_update(
        &self,
        id: &str,
        data: &str,
    ) -> Result<SuccessMessages, ErrorsMessages> {
        ghostdb::systemcore::systemcore::update(id, String::from(data), &self.storage_driver).await
    }

    pub async fn command_remove(&self, id: &str) -> Result<SuccessMessages, ErrorsMessages> {
        ghostdb::systemcore::systemcore::remove(id, &self.storage_driver).await
    }
}
