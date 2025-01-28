use serde::{ Serialize, Deserialize};
use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Property {
    pub id: Uuid,
    pub address: String,
    pub price: BigDecimal,
    pub bedrooms: i32,
    pub bathrooms: i32,
}