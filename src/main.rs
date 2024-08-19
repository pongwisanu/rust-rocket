#[macro_use]
extern crate rocket;

mod db;
mod handlers;
mod models;
mod routes;
mod services;


#[launch]
fn rocket() -> _ {
    let db = db::init_db();
    rocket::build()
        .manage(db)
        .mount("/api", routes::get_routes())
}
