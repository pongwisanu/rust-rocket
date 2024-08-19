use rocket::http::Status;

use crate::{
    db::Db,
    models::{
        customers::{Customer, UpdateCustomer},
        default::Error,
    },
};

pub async fn get_users(db: &Db) -> Result<Vec<Customer>, Error> {
    let customers = db.lock().await;

    if customers.is_empty() {
        return Err(Error::NotFound);
    }

    Ok(customers.clone())
}

pub async fn get_user(db: &Db, id: i32) -> Result<Customer, Error> {
    let customers = db.lock().await;

    for customer in customers.iter() {
        if customer.id == id {
            return Ok(customer.clone());
        }
    }

    Err(Error::NotFound)
}

pub async fn create_user(db: &Db, new_customer: Customer) -> Result<Status, Error> {
    let mut customers = db.lock().await;

    for customer in customers.iter() {
        if customer.id == new_customer.id {
            return Err(Error::BadRequest);
        }
    }

    customers.push(new_customer);

    Ok(Status::Created)
}

pub async fn update_user(
    db: &Db,
    id: i32,
    update_customer: UpdateCustomer,
) -> Result<Status, Error> {
    let mut customers = db.lock().await;

    for customer in customers.iter_mut() {
        if customer.id == id {
            customer.name = update_customer.name;
            return Ok(Status::Ok);
        }
    }

    Err(Error::NotFound)
}

pub async fn delete_user(db: &Db, id: i32) -> Result<Status, Error> {
    let mut customers = db.lock().await;

    let customer_count = customers.len();

    customers.retain(|customer| customer.id != id);

    let deleted = customers.len() != customer_count;
    if deleted {
        Ok(Status::NoContent)
    } else {
        Err(Error::NotFound)
    }
}
