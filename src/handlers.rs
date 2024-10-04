use crate::db::DbOps;
use crate::external::get_geolocation;
use crate::utils::serialize_geolocation_data;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use tracing::{info, warn};

pub async fn health_checker_handler() -> impl IntoResponse {
    // Healthcheck response
    (StatusCode::OK, "Alive")
}

// Big function, needs to be split into smaller functions
pub async fn get_ip(
    Path(ip): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Try to get IP data from the database
    match app_state.db.get_ip(ip.clone()).await {
        Ok(Some(ip_geolocation)) => {
            // If IP data exists, return it as JSON
            info!("Ip {} already registered", &ip);
            return Ok((
                StatusCode::OK,
                Json(serde_json::to_value(ip_geolocation).unwrap()),
            ));
        }
        Ok(None) => {
            info!("Ip {} not found in database", &ip);
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
    match get_geolocation(&ip).await {
        Ok(ip_geolocation) => {
            info!("Retriveing geolocation data for {}", &ip);

            // Serialize the geolocation data to validate its structure
            match serialize_geolocation_data(&ip_geolocation.to_string()) {
                Ok(data) => {
                    info!("Geolocation data serialized successfully");

                    // Try to insert the geolocation data into the database
                    match app_state.db.insert_ip(&data).await {
                        Ok(_) => info!("Ip {} registered in database", ip),
                        Err(e) => warn!("Error inserting IP data into database: {}", e),
                    }

                    // Return the original JSON from the external service
                    Ok((StatusCode::OK, ip_geolocation))
                }
                Err(e) => {
                    warn!(
                        "Error serializing geolocation data: {}. External api response {}",
                        e,
                        ip_geolocation.to_string()
                    );
                    // Return the original JSON from the external API if serialization fails
                    Ok((StatusCode::NOT_FOUND, ip_geolocation))
                }
            }
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
