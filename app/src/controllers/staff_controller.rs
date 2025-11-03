use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::staff::{Staff, CreateStaffRequest, UpdateStaffRequest};

#[post("/")]
pub async fn create_staff(pool: web::Data<SqlitePool>, staff: web::Json<CreateStaffRequest>) -> impl Responder {
    match sqlx::query("INSERT INTO staff (first_name, last_name, role, phone) VALUES (?, ?, ?, ?)")
        .bind(&staff.first_name)
        .bind(&staff.last_name)
        .bind(&staff.role)
        .bind(&staff.phone)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            let new_staff = Staff {
                id: result.last_insert_rowid(),
                first_name: staff.first_name.clone(),
                last_name: staff.last_name.clone(),
                role: staff.role.clone(),
                phone: staff.phone.clone(),
            };
            HttpResponse::Created().json(new_staff)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/")]
pub async fn get_staff(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT id, first_name, last_name, role, phone FROM staff ORDER BY id")
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let staff: Vec<Staff> = rows.iter().map(|row| Staff {
                id: row.get("id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                role: row.get("role"),
                phone: row.get("phone"),
            }).collect();
            HttpResponse::Ok().json(staff)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/{id}")]
pub async fn get_staff_by_id(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("SELECT id, first_name, last_name, role, phone FROM staff WHERE id = ?")
        .bind(id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(row)) => {
            let staff = Staff {
                id: row.get("id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                role: row.get("role"),
                phone: row.get("phone"),
            };
            HttpResponse::Ok().json(staff)
        }
        Ok(None) => HttpResponse::NotFound().body("Staff not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/{id}")]
pub async fn update_staff(pool: web::Data<SqlitePool>, path: web::Path<i64>, updated: web::Json<UpdateStaffRequest>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("UPDATE staff SET first_name = ?, last_name = ?, role = ?, phone = ? WHERE id = ?")
        .bind(&updated.first_name)
        .bind(&updated.last_name)
        .bind(&updated.role)
        .bind(&updated.phone)
        .bind(id)
        .execute(&**pool)
        .await
    {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().body("Updated"),
        Ok(_) => HttpResponse::NotFound().body("Staff not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/{id}")]
pub async fn delete_staff(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query("DELETE FROM staff WHERE id = ?")
        .bind(id)
        .execute(&**pool)
        .await
    {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().body("Staff not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/role/{role}")]
pub async fn get_staff_by_role(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>
) -> impl Responder {
    let role: String = path.into_inner();

    match sqlx::query("SELECT * FROM staff WHERE role = ?")
        .bind(role)
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let staff: Vec<Staff> = rows.iter().map(|row| Staff {
                id: row.get("id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                role: row.get("role"),
                phone: row.get("phone")
            }).collect();

            HttpResponse::Ok().json(staff)
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Database error: {}", e))
    }
    
}