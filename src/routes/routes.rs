use actix_web::{delete, get, HttpResponse, post, put, web};
use actix_web::web::{Json, Path};
use crate::infrastructure::Db;
use crate::infrastructure::external_query::get_geolocation;
use crate::infrastructure::mongodb::DbOps;
use crate::models::Ip;

#[post("/insert")]
pub async fn insert_ip(db: web::Data<Db>, ip: Json<Ip>) -> HttpResponse {
    let data = ip.into_inner();

    // Get external data of ip geolocation
    let ip_geolocation = get_geolocation(&data.ip).await.expect("");

    let result = db.insert_ip(&ip_geolocation).await;
    match result {
        Ok(ip_id) => HttpResponse::Ok().body(format!("Ip {} saved with mongo uuid: {}", data.ip, ip_id.to_hex())),
        Err(_) => HttpResponse::InternalServerError().body("Error to insert the IP"),
    }
}

#[get("/get/{ip}")]
pub async fn get_ip(db: web::Data<Db>, ip: Path<String>) -> HttpResponse {
    let ip = ip.into_inner();
    let result = db.get_ip(ip).await;

    match result {
        Ok(Some(ip)) => HttpResponse::Ok().json(ip),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No data found for requested data"))
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting IP address"),
    }
}

#[put("/update/{ip}")]
async fn update_ip(db: web::Data<Db>, ip: Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("Ip updated :)")
}

#[delete("/delete/{ip}")]
pub async fn delete_ip(db: web::Data<Db>, ip: Path<String>) -> HttpResponse{
    let ip = ip.into_inner();
    let result = db.delete_ip(ip).await;

    match result {
        Ok(count) => HttpResponse::Ok().body(format!("Ip deleted. Count: {}", count.deleted_count)),
        Err(_) =>  HttpResponse::InternalServerError().body("Error deleting IP address"),
    }
}

// Healthcheck path, unauthenticated
#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Alive!")
}