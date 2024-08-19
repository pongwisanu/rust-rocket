use crate::{
    db::Db,
    models::{
        customers::{Customer, UpdateCustomer},
        default::Error,
    },
    services,
};
use rocket::{
    http::Status,
    response::status::{self, Custom},
    serde::json::Json,
    Route, State,
};

pub fn get_routes() -> Vec<Route> {
    routes![
        get_customers,
        get_customer,
        post_customer,
        put_customer,
        delete_customer
    ]
}

#[get("/customers")]
pub async fn get_customers(db: &State<Db>) -> Result<Json<Vec<Customer>>, Custom<String>> {
    match services::customers::get_users(db).await {
        Ok(customers) => Ok(Json(customers)),
        Err(Error::NotFound) => Err(status::Custom(
            Status::NotFound,
            "No customers found".to_string(),
        )),
        _ => Err(status::Custom(
            Status::InternalServerError,
            "Internal Server error".to_string()
        ))
    }
}

#[get("/customers/<id>")]
pub async fn get_customer(db: &State<Db>, id: i32) -> Result<Json<Customer>, Custom<String>> {
    match services::customers::get_user(db, id).await {
        Ok(customers) => Ok(Json(customers)),
        Err(Error::NotFound) => Err(status::Custom(
            Status::NotFound,
            "No customer request found".to_string(),
        )),
        Err(Error::BadRequest) => Err(status::Custom(
            Status::BadRequest,
            "Bad Request".to_string(),
        )),
    }
}

#[post("/customers", data = "<new_customer>")]
pub async fn post_customer(
    db: &State<Db>,
    new_customer: Json<Customer>,
) -> Result<Custom<String>, Custom<String>> {
    match services::customers::create_user(db, new_customer.into_inner()).await {
        Ok(_) => Ok(status::Custom(
            Status::Created,
            "Create Successful".to_string(),
        )),
        Err(Error::BadRequest) => Err(status::Custom(
            Status::BadRequest,
            "Bad Request".to_string(),
        )),
        _ => Err(status::Custom(
            Status::InternalServerError,
            "Internal Server error".to_string()
        ))
    }
}

#[put("/customers/<id>", data = "<update_customer>")]
pub async fn put_customer(
    db: &State<Db>,
    id: i32,
    update_customer: Json<UpdateCustomer>,
) -> Result<Custom<String>, Custom<String>> {
    match services::customers::update_user(db, id, update_customer.into_inner()).await {
        Ok(_) => Ok(status::Custom(
            Status::Created,
            "Update Successful".to_string(),
        )),
        Err(Error::NotFound) => Err(status::Custom(
            Status::NotFound,
            "No customer request found".to_string(),
        )),
        Err(Error::BadRequest) => Err(status::Custom(
            Status::BadRequest,
            "Bad Request".to_string(),
        )),
    }
}

#[delete("/customers/<id>")]
pub async fn delete_customer(db: &State<Db>, id: i32) -> Result<Custom<String>, Custom<String>> {
    match services::customers::delete_user(db, id).await {
        Ok(_) => Ok(status::Custom(
            Status::Created,
            "Delete Successful".to_string(),
        )),
        Err(Error::NotFound) => Err(status::Custom(
            Status::NotFound,
            "No customer request found".to_string(),
        )),
        _ => Err(status::Custom(
            Status::InternalServerError,
            "Internal Server error".to_string()
        ))
    }
}
