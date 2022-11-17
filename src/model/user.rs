
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct User{
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
}