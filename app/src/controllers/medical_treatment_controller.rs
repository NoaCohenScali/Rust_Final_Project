use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::medical_treatment::{CreateMedicalTreatmentRequest, MedicalTreatment, UpdateMedicalTreatmentRequest};

#[post("/")]
pub async fn create_medical_treatment(
    pool: web::Data<SqlitePool>,
    treatment: web::Json<CreateMedicalTreatmentRequest>,
) -> impl Responder {
    match sqlx::query(
        "INSERT INTO medical_treatments (animal_id, staff_id, date, description) VALUES (?, ?, ?, ?)",
    )
    .bind(&treatment.animal_id)
    .bind(&treatment.staff_id)
    .bind(&treatment.date)
    .bind(&treatment.description)
    .execute(&**pool)
    .await
    {
        Ok(result) => {
            let new_treatment = MedicalTreatment {
                id: result.last_insert_rowid(),
                animal_id: treatment.animal_id,
                staff_id: treatment.staff_id,
                date: treatment.date,
                description: treatment.description.clone(),
            };
            HttpResponse::Created().json(new_treatment)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/")]
pub async fn get_medical_treatments(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, animal_id, staff_id, date, description FROM medical_treatments ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let treatments: Vec<MedicalTreatment> = rows
                .iter()
                .map(|row| MedicalTreatment {
                    id: row.get("id"),
                    animal_id: row.get("animal_id"),
                    staff_id: row.get("staff_id"),
                    date: row.get("date"),
                    description: row.get("description"),
                })
                .collect();
            HttpResponse::Ok().json(treatments)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/{id}")]
pub async fn get_medical_treatment_by_id(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("SELECT id, animal_id, staff_id, date, description FROM medical_treatments WHERE id = ?")
        .bind(&id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(row)) => {
            let treatment = MedicalTreatment {
                id: row.get("id"),
                animal_id: row.get("animal_id"),
                staff_id: row.get("staff_id"),
                date: row.get("date"),
                description: row.get("description"),
            };
            HttpResponse::Ok().json(treatment)
        }
        Ok(None) => HttpResponse::NotFound().body("Medical treatment not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/{id}")]
pub async fn update_medical_treatment(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    updated: web::Json<UpdateMedicalTreatmentRequest>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("UPDATE medical_treatments SET animal_id = ?, staff_id = ?, date = ?, description = ? WHERE id = ?")
        .bind(&updated.animal_id)
        .bind(&updated.staff_id)
        .bind(&updated.date)
        .bind(&updated.description)
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let updated_treatment = MedicalTreatment {
                    id: id,
                    animal_id: updated.animal_id,
                    staff_id: updated.staff_id,
                    date: updated.date,
                    description: updated.description.clone(),
                };
                HttpResponse::Ok().json(updated_treatment)
            } else {
                HttpResponse::NotFound().body("Medical treatment not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/{id}")]
pub async fn delete_medical_treatment(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("DELETE FROM medical_treatments WHERE id = ?")
        .bind(&id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Deleted successfully")
            } else {
                HttpResponse::NotFound().body("Medical treatment not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/by_staff/{first_name}/{last_name}")]
pub async fn get_medical_treatment_by_staff(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, String)>
) -> impl Responder {
    let (first_name, last_name) = path.into_inner();

    match sqlx::query("SELECT m.* FROM medical_treatments m 
        JOIN staff s ON m.staff_id = s.id 
        WHERE s.first_name = ? AND s.last_name = ?")
        .bind(first_name)
        .bind(last_name)
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let medical_treatments: Vec<MedicalTreatment> = rows.iter().map(|row| MedicalTreatment {
                id: row.get("id"),
                animal_id: row.get("animal_id"),
                staff_id: row.get("staff_id"),
                date: row.get("date"),
                description: row.get("description")
            }).collect();

            HttpResponse::Ok().json(medical_treatments)
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Database error: {}", e))
    }
    
}
