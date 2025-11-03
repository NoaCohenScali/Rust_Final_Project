use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use chrono::NaiveDateTime;
use crate::models::feeding::{Feeding, CreateFeedingRequest, UpdateFeedingRequest};

#[post("/")]
pub async fn create_feeding(pool: web::Data<SqlitePool>, feeding: web::Json<CreateFeedingRequest>) -> impl Responder {
    match sqlx::query("INSERT INTO feedings (animal_id, staff_id, feeding_time) VALUES (?, ?, ?)")
        .bind(&feeding.animal_id)
        .bind(&feeding.staff_id)
        .bind(&feeding.feeding_time)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            let new_feed = Feeding {
                id: result.last_insert_rowid(),
                animal_id: feeding.animal_id.clone(),
                staff_id: feeding.staff_id.clone(),
                feeding_time: feeding.feeding_time.clone(),
            };
            HttpResponse::Created().json(new_feed)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/")]
pub async fn get_feedings(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, animal_id, staff_id, feeding_time FROM feedings ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let feedings: Vec<Feeding> = rows.iter().map(|row| Feeding {
                id: row.get("id"),
                animal_id: row.get("animal_id"),
                staff_id: row.get("staff_id"),
                feeding_time: row.get("feeding_time"),
            }).collect();
            HttpResponse::Ok().json(feedings)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/{id}")]
pub async fn get_feeding_by_id(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("SELECT id, animal_id, staff_id, feeding_time FROM feedings WHERE id = ?")
        .bind(id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(row)) => {
            let feeding = Feeding {
                id: row.get("id"),
                animal_id: row.get("animal_id"),
                staff_id: row.get("staff_id"),
                feeding_time: row.get("feeding_time"),
            };
            HttpResponse::Ok().json(feeding)
        }
        Ok(None) => HttpResponse::NotFound().body("Feeding not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/{id}")]
pub async fn update_feeding(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateFeedingRequest>,
) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query("UPDATE feedings SET animal_id = ?, staff_id = ?, feeding_time = ? WHERE id = ?")
        .bind(&updated.animal_id)
        .bind(&updated.staff_id)
        .bind(&updated.feeding_time)
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let updated_feeding = Feeding {
                    id: id,
                    animal_id: updated.animal_id.clone(),
                    staff_id: updated.staff_id.clone(),
                    feeding_time: updated.feeding_time,
                };
                HttpResponse::Ok().json(updated_feeding)
            } else {
                HttpResponse::NotFound().body("Feeding not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/{id}")]
pub async fn delete_feeding(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("DELETE FROM feedings WHERE id = ?")
        .bind(id)
        .execute(&**pool)
        .await
    {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Feeding not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/after/{date}")]
pub async fn get_feedings_after(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> impl Responder {
    let date_str = path.into_inner();

    let date_limit = match NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S") {
        Ok(dt) => dt,
        Err(_) => return HttpResponse::BadRequest()
            .body("Invalid date format (use YYYY-MM-DDTHH:MM:SS)"),
    };

    match sqlx::query("SELECT id, animal_id, staff_id, feeding_time FROM feedings WHERE feeding_time > ? ORDER BY feeding_time")
        .bind(date_limit)
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let feedings: Vec<Feeding> = rows.iter().map(|row| Feeding {
                id: row.get("id"),
                animal_id: row.get("animal_id"),
                staff_id: row.get("staff_id"),
                feeding_time: row.get("feeding_time"),
            }).collect();

            HttpResponse::Ok().json(feedings)
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Database error: {}", e)),
    }
}
