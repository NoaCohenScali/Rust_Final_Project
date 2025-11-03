use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::enclosure::{Enclosure, CreateEnclosureRequest, UpdateEnclosureRequest};

#[post("/")]
pub async fn create_enclosure(pool: web::Data<SqlitePool>, enclosure: web::Json<CreateEnclosureRequest>) -> impl Responder {
    match sqlx::query("INSERT INTO enclosures (name, enclosure_type, capacity) VALUES (?, ?, ?)")
        .bind(&enclosure.name)
        .bind(&enclosure.enclosure_type)
        .bind(&enclosure.capacity)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            let new_enclosure = Enclosure {
                id: result.last_insert_rowid(),
                name: enclosure.name.clone(),
                enclosure_type: enclosure.enclosure_type.clone(),
                capacity: enclosure.capacity.clone(),
            };
            HttpResponse::Created().json(new_enclosure)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/")]
pub async fn get_enclosures(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, name, enclosure_type, capacity FROM enclosures ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let enclosures: Vec<Enclosure> = rows.iter().map(|row| Enclosure {
                id: row.get("id"),
                name: row.get("name"),
                enclosure_type: row.get("enclosure_type"),
                capacity: row.get("capacity"),
            }).collect();
            HttpResponse::Ok().json(enclosures)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/full")]
pub async fn get_full_enclosure(
    pool: web::Data<SqlitePool>
) -> impl Responder {
    match sqlx::query("SELECT e.* FROM enclosures e 
        JOIN animals a ON e.id = a.enclosure_id 
        GROUP BY a.enclosure_id
        HAVING COUNT(a.enclosure_id) = e.capacity")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let enclosures: Vec<Enclosure> = rows.iter().map(|row| Enclosure {
                id: row.get("id"),
                name: row.get("name"),
                enclosure_type: row.get("enclosure_type"),
                capacity: row.get("capacity")
            }).collect();

            HttpResponse::Ok().json(enclosures)
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Database error: {}", e))
    }
}

#[get("/{id}")]
pub async fn get_enclosure_by_id(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("SELECT id, name, enclosure_type, capacity FROM enclosures WHERE id = ?")
        .bind(&id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(row)) => {
            let enclosure = Enclosure {
                id: row.get("id"),
                name: row.get("name"),
                enclosure_type: row.get("enclosure_type"),
                capacity: row.get("capacity"),
            };
            HttpResponse::Ok().json(enclosure)
        }
        Ok(None) => HttpResponse::NotFound().body("Enclosure not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/{id}")]
pub async fn update_enclosure(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateEnclosureRequest>,
) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("UPDATE enclosures SET name = ?, enclosure_type = ?, capacity = ? WHERE id = ?")
        .bind(&updated.name)
        .bind(&updated.enclosure_type)
        .bind(&updated.capacity)
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let updated_enclosure = Enclosure {
                    id: id,
                    name: updated.name.clone(),
                    enclosure_type: updated.enclosure_type.clone(),
                    capacity: updated.capacity,
                };
                HttpResponse::Ok().json(updated_enclosure)
            } else {
                HttpResponse::NotFound().body("Enclosure not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/{id}")]
pub async fn delete_enclosure(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("DELETE FROM enclosures WHERE id = ?")
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Deleted successfully")
            } else {
                HttpResponse::NotFound().body("Enclosure not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

