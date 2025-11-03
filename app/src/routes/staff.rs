use actix_web::{web, Scope};
use crate::controllers::staff_controller;

pub fn staff_routes() -> Scope {
    web::scope("/staff")
        .service(staff_controller::create_staff)
        .service(staff_controller::get_staff)
        .service(staff_controller::get_staff_by_id)
        .service(staff_controller::update_staff)
        .service(staff_controller::delete_staff)
        .service(staff_controller::get_staff_by_role)
}