use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Staff {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub phone: String
}

#[derive(Debug, Deserialize)]
pub struct CreateStaffRequest {
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub phone: String
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffRequest {
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub phone: String
}