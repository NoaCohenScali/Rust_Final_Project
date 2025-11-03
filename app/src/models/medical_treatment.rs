use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalTreatment {
    pub id: i64,
    pub animal_id: i64,
    pub staff_id: i64,
    pub date: NaiveDateTime,
    pub description: String
}

#[derive(Debug, Deserialize)]
pub struct CreateMedicalTreatmentRequest {
    pub animal_id: i64,
    pub staff_id: i64,
    pub date: NaiveDateTime,
    pub description: String
}

#[derive(Debug, Deserialize)]
pub struct UpdateMedicalTreatmentRequest {
    pub animal_id: i64,
    pub staff_id: i64,
    pub date: NaiveDateTime,
    pub description: String
}