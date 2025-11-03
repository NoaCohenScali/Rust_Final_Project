use actix_web::{web, Scope};
use crate::controllers::animal_controller;

pub fn animal_routes() -> Scope {
    web::scope("/animals")
        .service(animal_controller::create_animal)
        .service(animal_controller::get_animals)
        .service(animal_controller::get_animal_by_id)
        .service(animal_controller::update_animal)
        .service(animal_controller::delete_animal)
        .service(animal_controller::get_animals_older_than)
}