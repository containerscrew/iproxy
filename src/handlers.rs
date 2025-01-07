use crate::db::DbOps;
use crate::external::get_geolocation;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use tracing::{debug, warn};

pub async fn health_checker_handler() -> impl IntoResponse {
    // Healthcheck response
    (StatusCode::OK, "Alive")
}

// Big function, needs to be split into smaller functions
pub async fn get_ip(
    Path(ip): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    debug!("ip requested: {}", &ip);

    // Try to get IP data from the database
    match app_state.db.get_ip(ip.clone()).await {
        Ok(Some(ip_geolocation)) => {
            // If IP data exists, return it as JSON
            debug!("ip {} already registered in database", &ip);
            return Ok((
                StatusCode::OK,
                Json(serde_json::from_value(ip_geolocation).unwrap()),
            ));
        }
        Ok(None) => {
            debug!("ip {} not found in database", &ip);
        }
        Err(e) => {
            warn!("Error getting ip data: {}", e);
            let db_error = serde_json::json!({
                "error": "Failed to retrieve IP from the database",
                "details": e.to_string()
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(db_error)));
        }
    }

    // If IP data does not exist in the database, get it from the external service
    match get_geolocation(&ip, app_state.use_proxy).await {
        Ok(Json(ip_geolocation)) => {
            debug!("retriveing geolocation data for {}", &ip);
            
            // Try to insert the geolocation data into the database
            match app_state.db.insert_ip(&ip_geolocation).await {
                Ok(_) => debug!("Ip {} registered in database", ip),
                Err(e) => warn!("Error inserting IP data into database: {}", e),
            }

            // Return the original JSON from the external service
            Ok((StatusCode::OK, Json(ip_geolocation)))
        }
        Err(e) => {
            // Handle errors when calling the external geolocation service
            warn!("Error retrieving geolocation data: {}", e);
            let geolocation_error = serde_json::json!({
                "error": "Failed to retrieve geolocation data",
                "details": e.to_string()
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(geolocation_error)))
        }
    }
}

pub async fn handler_404() -> impl IntoResponse {
    // Default 404 not found
    (StatusCode::NOT_FOUND, "404 Not Found")
}
