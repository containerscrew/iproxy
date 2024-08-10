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