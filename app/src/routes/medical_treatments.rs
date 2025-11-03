use actix_web::{web, Scope};
use crate::controllers::medical_treatment_controller;

pub fn medical_treatment_routes() -> Scope {
    web::scope("/medical_treatments")
        .service(medical_treatment_controller::create_medical_treatment)
        .service(medical_treatment_controller::get_medical_treatments)
        .service(medical_treatment_controller::get_medical_treatment_by_id)
        .service(medical_treatment_controller::update_medical_treatment)
        .service(medical_treatment_controller::delete_medical_treatment)
        .service(medical_treatment_controller::get_medical_treatment_by_staff)
}