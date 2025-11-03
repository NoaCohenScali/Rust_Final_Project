use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feeding {
    pub id: i64,
    pub animal_id: i64,
    pub staff_id: i64,
    pub feeding_time: NaiveDateTime
}

#[derive(Debug, Deserialize)]
pub struct CreateFeedingRequest {
    pub animal_id: i64,
    pub staff_id: i64,
    pub feeding_time: NaiveDateTime
}

#[derive(Debug, Deserialize)]
pub struct UpdateFeedingRequest {
   pub animal_id: i64,
    pub staff_id: i64,
    pub feeding_time: NaiveDateTime
}