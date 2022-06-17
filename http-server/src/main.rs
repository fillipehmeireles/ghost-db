#[macro_use]
extern crate rocket;

use controllers::indexcontrollers;

use providers::DbProvider;

mod controllers;
mod models;
mod providers;

#[launch]
async fn rocket() -> _ {
    let db_provider: DbProvider = DbProvider::new().await;

    rocket::build().manage(db_provider).mount(
        "/",
        routes![
            indexcontrollers::get_all,
            indexcontrollers::insert,
            indexcontrollers::get_one,
            indexcontrollers::update,
            indexcontrollers::delete
        ],
    )
}
