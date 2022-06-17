use crate::models::DataTraffic;
use crate::providers::DbProvider;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde_derive::{Deserialize, Serialize};

#[get("/get-all")]
pub fn get_all(db_provider: &State<DbProvider>) -> Result<status::Accepted<String>, Status> {
    let all_data = db_provider.get_all();
    match serde_json::to_string(&all_data) {
        Ok(result) => Ok(status::Accepted(Some(result))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get-one/<id>")]
pub fn get_one(id: &str, db_provider: &State<DbProvider>) -> Json<DataTraffic> {
    match db_provider.get_one(id) {
        Some(data) => Json(DataTraffic {
            key: id.to_string(),
            value: data,
        }),
        None => Json(DataTraffic {
            key: String::from(""),
            value: String::from(""),
        }),
    }
}

#[post("/insert", format = "json", data = "<data>")]
pub async fn insert(
    data: Json<DataTraffic>,
    db_provider: &State<DbProvider>,
) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let data_inner = data.into_inner();
    let inserted_data = db_provider
        .insert(DataTraffic {
            key: data_inner.key,
            value: data_inner.value,
        })
        .await;
    match inserted_data {
        Ok(result) => Ok(status::Accepted(Some(result.to_string()))),
        Err(result_err) => Err(status::BadRequest(Some(result_err.to_string()))),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RequestUpdate {
    value: String,
}

#[put("/update/<id>", data = "<data>")]
pub async fn update(
    data: Json<RequestUpdate>,
    id: &str,
    db_provider: &State<DbProvider>,
) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    let data_inner = data.into_inner();
    match db_provider.update(id, data_inner.value).await {
        Ok(result) => Ok(status::Accepted(Some(result.to_string()))),
        Err(result_err) => Err(status::BadRequest(Some(result_err.to_string()))),
    }
}

#[delete("/delete/<id>")]
pub async fn delete(
    id: &str,
    db_provider: &State<DbProvider>,
) -> Result<status::Accepted<String>, status::BadRequest<String>> {
    match db_provider.remove(id).await {
        Ok(result) => Ok(status::Accepted(Some(result.to_string()))),
        Err(result_err) => Err(status::BadRequest(Some(result_err.to_string()))),
    }
}
