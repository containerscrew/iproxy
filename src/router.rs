use crate::handlers::{get_ip, handler_404, health_checker_handler};
use crate::AppState;
use axum::extract::ConnectInfo;
use axum::http::Request;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

const API_V1_BASE: &str = "/api/v1";

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &(API_V1_BASE.to_string() + "/health"),
            get(health_checker_handler),
        )
        .route(&(API_V1_BASE.to_string() + "/{ip}"), get(get_ip))
        .with_state(app_state)
        .fallback(handler_404)
        .layer(TraceLayer::new_for_http().on_request(
            |request: &Request<_>, _span: &tracing::Span| {
                let user_agent = request
                    .headers()
                    .get(axum::http::header::USER_AGENT)
                    .and_then(|value| value.to_str().ok())
                    .unwrap_or("Unknown");

                // Log the client IP from ConnectInfo
                if let Some(ConnectInfo(addr)) =
                    request.extensions().get::<ConnectInfo<SocketAddr>>()
                {
                    info!(
                        method = %request.method(),
                        uri = %request.uri(),
                        user_agent = %user_agent,
                        client_ip = %addr.ip(),
                        "incoming request"
                    );
                } else {
                    info!(
                        method = %request.method(),
                        uri = %request.uri(),
                        user_agent = %user_agent,
                        "incoming request (no client IP)"
                    );
                }
            },
        ))
}
