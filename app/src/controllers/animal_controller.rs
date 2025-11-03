use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::animal::{Animal, CreateAnimalRequest, UpdateAnimalRequest};

#[post("/")]
pub async fn create_animal(pool: web::Data<SqlitePool>, animal: web::Json<CreateAnimalRequest>) -> impl Responder {
    match sqlx::query("INSERT INTO animals (name, age, enclosure_id, species) VALUES (?, ?, ?, ?)")
        .bind(&animal.name)
        .bind(&animal.age)
        .bind(&animal.enclosure_id)
        .bind(&animal.species)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            let new_animal = Animal {
                id: result.last_insert_rowid(),
                name: animal.name.clone(),
                age: animal.age.clone(),
                enclosure_id: animal.enclosure_id.clone(),
                species: animal.species.clone(),
            };
            HttpResponse::Created().json(new_animal)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/")]
pub async fn get_animals(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, name, age, enclosure_id, species FROM animals ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let animals: Vec<Animal> = rows
                .iter()
                .map(|row| Animal {
                    id: row.get("id"),
                    name: row.get("name"),
                    age: row.get("age"),
                    enclosure_id: row.get("enclosure_id"),
                    species: row.get("species"),
                })
                .collect();
            HttpResponse::Ok().json(animals)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/{id}")]
pub async fn get_animal_by_id(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query("SELECT id, name, age, enclosure_id, species FROM animals WHERE id = ?")
        .bind(&id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(row)) => {
            let animal = Animal {
                id: row.get("id"),
                name: row.get("name"),
                age: row.get("age"),
                enclosure_id: row.get("enclosure_id"),
                species: row.get("species"),
            };
            HttpResponse::Ok().json(animal)
        }
        Ok(None) => HttpResponse::NotFound().body("Animal not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/{id}")]
pub async fn update_animal(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateAnimalRequest>,
) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query("UPDATE animals SET name = ?, age = ?, enclosure_id = ?, species = ? WHERE id = ?")
        .bind(&updated.name)
        .bind(&updated.age)
        .bind(&updated.enclosure_id)
        .bind(&updated.species)
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let updated_animal = Animal {
                    id: id,
                    name: updated.name.clone(),
                    age: updated.age.clone(),
                    enclosure_id: updated.enclosure_id.clone(),
                    species: updated.species.clone(),
                };
                HttpResponse::Ok().json(updated_animal)
            } else {
                HttpResponse::NotFound().body("Animal not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/{id}")]
pub async fn delete_animal(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query("DELETE FROM animals WHERE id = ?")
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Deleted")
            } else {
                HttpResponse::NotFound().body("Animal not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/older/{age}")]
pub async fn get_animals_older_than(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>
) -> impl Responder {
    let age_str = path.into_inner();
    let age: i32 = match age_str.parse() {
        Ok(a) => a,
        Err(_) => return HttpResponse::BadRequest().body("Invalid age format"),
    };

    match sqlx::query("SELECT id, name, age, enclosure_id, species FROM animals WHERE age > ?")
        .bind(age)
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let animals: Vec<Animal> = rows.iter().map(|row| Animal {
                id: row.get("id"),
                name: row.get("name"),
                age: row.get("age"),
                enclosure_id: row.get("enclosure_id"),
                species: row.get("species")
            }).collect();

            HttpResponse::Ok().json(animals)
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Database error: {}", e))
    }
}