use actix_web::{web, Scope};
use crate::controllers::feeding_controller;

pub fn feeding_routes() -> Scope {
    web::scope("/feedings")
        .service(feeding_controller::create_feeding)
        .service(feeding_controller::get_feedings)
        .service(feeding_controller::get_feeding_by_id)
        .service(feeding_controller::update_feeding)
        .service(feeding_controller::delete_feeding)
        .service(feeding_controller::get_feedings_after)
}