use axum::extract::{Path};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::external::get_geolocation;

pub async fn health_checker_handler() -> impl IntoResponse {
    // Healthcheck response
    (StatusCode::OK, "Alive")
}

pub async fn get_ip(Path(ip): Path<String>) -> impl IntoResponse {
    // Get ip data from external service.
    let ip_geolocation = get_geolocation(&ip).await.expect("");

    // TODO: save data into mongodb

    // Return status code and ip data
    (StatusCode::OK, Json(ip_geolocation))
}

pub async fn handler_404() -> impl IntoResponse {
    // Default 404 not found
    (StatusCode::NOT_FOUND, "404 Not Found")
}