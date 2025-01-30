use serde::{ Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Booking {
    pub id: Uuid,
    pub property_id: Uuid,
    pub user_id: Uuid,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}