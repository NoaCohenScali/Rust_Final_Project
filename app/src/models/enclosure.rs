use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Enclosure {
    pub id: i64,
    pub name: String,
    pub enclosure_type: String,
    pub capacity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateEnclosureRequest {
    pub name: String,
    pub enclosure_type: String,
    pub capacity: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEnclosureRequest {
   pub name: String,
    pub enclosure_type: String,
    pub capacity: i32,
}