use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("No customers found")]
    NotFound,
    #[error("Bad Request")]
    BadRequest,
}
