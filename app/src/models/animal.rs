use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Animal {
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub enclosure_id: i64,
    pub species: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnimalRequest {
    pub name: String,
    pub age: i32,
    pub enclosure_id: i64,
    pub species: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAnimalRequest {
    pub name: String,
    pub age: i32,
    pub enclosure_id: i64,
    pub species: String,
}