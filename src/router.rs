use crate::handlers::{get_ip, health_checker_handler};
use crate::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

const API_V1_BASE: &str = "/api/v1";

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &(API_V1_BASE.to_string() + "/health"),
            get(health_checker_handler),
        )
        .route(&(API_V1_BASE.to_string() + "/:ip"), get(get_ip))
        .with_state(app_state)
}
