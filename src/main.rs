use crate::config::Config;
use crate::db::Db;
// use crate::error::MyError;
use crate::handlers::handler_404;
use crate::logger::setup_logger;
use crate::router::create_router;
use std::sync::Arc;
use tracing::info;

mod config;
mod db;
mod error;
mod external;
mod handlers;
mod logger;
mod models;
mod router;
mod utils;

pub struct AppState {
    db: Db,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Determine which configuration file to load based on the IS_LOCAL environment variable
    let config_file = if std::env::var("IS_LOCAL").unwrap_or_else(|_| "false".to_string()) == "true"
    {
        "local.config.toml"
    } else {
        "config.toml"
    };
    // Load configuration file
    let config = Config::from_file(config_file);

    // Enable logging
    setup_logger(config.logging.log_level);

    // Init database
    let db = Db::init(
        config.database.endpoint,
        config.database.db_name,
        config.database.collection_name,
    )
    .await?;

    // TODO: control CORS

    // Run server
    let app = create_router(Arc::new(AppState { db: db.clone() })); //.layer(cors);
    let app = app.fallback(handler_404);

    // Create index
    db.create_ips_index().await;

    info!("Server started successfully!");

    let addr = format!("{}:{}", config.server.address, config.server.port);
    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
