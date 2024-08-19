use serde::{Deserialize , Serialize};

#[derive(Clone , Default , Debug ,  Deserialize, Serialize )]
pub struct Customer {
    pub id: i32,
    pub name: String,
}

#[derive(Clone , Default , Debug , Deserialize , Serialize)]
pub  struct UpdateCustomer{
    pub name: String
}