use actix_web::{web, App, HttpServer};
use crate::{db::init_db, routes::{animals::animal_routes, enclosures::enclosure_routes, feedings::feeding_routes, medical_treatments::medical_treatment_routes, staff::staff_routes}};

mod db;
mod models;
mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    let pool = init_db().await.expect("Failed to initialize database");
    
    println!("🚀 Server running at http://127.0.0.1:8080");
    println!("📊 SQLite database initialized at src/zoo.db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(animal_routes())
            .service(enclosure_routes())
            .service(feeding_routes())
            .service(medical_treatment_routes())
            .service(staff_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}