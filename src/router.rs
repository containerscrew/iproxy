use std::sync::Arc;
use axum::{
    routing::{get},
    Router,
};
use crate::AppState;
use crate::handlers::{get_ip, health_checker_handler};

// const API_V1_BASE: &str = "/api/v1";

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_checker_handler))
        .route("/ip/:ip", get(get_ip)
        )
        .with_state(app_state)
}