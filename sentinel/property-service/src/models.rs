use serde::{ Serialize, Deserialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Property {
    pub id: Uuid,
    pub address: String,
    pub value: f64,
    pub bedrooms: i32,
    pub bathrooms: i32,
}