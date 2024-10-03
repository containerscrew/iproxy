use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tracing::{info, warn};
use crate::AppState;
use crate::external::get_geolocation;
use crate::db::DbOps;

pub async fn health_checker_handler() -> impl IntoResponse {
    // Healthcheck response
    (StatusCode::OK, "Alive")
}

pub async fn get_ip(Path(ip): Path<String>, State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    // Get ip data from mongodb if exists
    match app_state.db.get_ip(ip.clone()).await {
        Ok(Some(ip_geolocation)) => {
            // Si la IP ya está en la base de datos, devuélvela
            info!("Ip {} already registered", &ip);
            return (StatusCode::OK, Json(ip_geolocation));
        }
        Ok(None) => {
            // Si no hay datos para la IP
            info!("Ip {} not found in database", &ip);
        }
        Err(e) => {
            // Si ocurre un error al obtener la IP
            warn!("Error getting ip data: {}", e);
        }
    }

    // Get ip data from external service.
    let ip_geolocation = get_geolocation(&ip).await.expect("");

    // TODO: save data into mongodb
    match app_state.db.insert_ip(&ip_geolocation).await {
        Ok(_) => info!("Ip {} registered", ip),
        Err(e) => warn!("Error inserting ip data, already exist: {}", e),
    }

    // Return status code and ip data
    (StatusCode::OK, Json(ip_geolocation))
}

pub async fn handler_404() -> impl IntoResponse {
    // Default 404 not found
    (StatusCode::NOT_FOUND, "404 Not Found")
}