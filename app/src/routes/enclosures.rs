use actix_web::{web, Scope};
use crate::controllers::enclosure_controller;

pub fn enclosure_routes() -> Scope {
    web::scope("/enclosures")
        .service(enclosure_controller::create_enclosure)
        .service(enclosure_controller::get_enclosures)
        .service(enclosure_controller::get_full_enclosure)
        .service(enclosure_controller::get_enclosure_by_id)
        .service(enclosure_controller::update_enclosure)
        .service(enclosure_controller::delete_enclosure)
}