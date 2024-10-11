use crate::config::Config;
use crate::db::Db;
// use crate::error::MyError;
use crate::handlers::handler_404;
use crate::logger::setup_logger;
use crate::router::create_router;
use std::net::SocketAddr;
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
    pub use_proxy: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration file. Set the CONFIG_FILE_PATH env var. Example: CONFIG_FILE_PATH=./config.toml
    let config = Config::load_config();

    // Enable logging
    setup_logger(config.logging.log_level);

    info!("starting iproxy server");

    // Init database
    let db = Db::init(
        config.database.endpoint,
        config.database.db_name,
        config.database.collection_name,
    )
    .await?;

    // TODO: control CORS

    // Run server
    let app = create_router(Arc::new(AppState {
        db: db.clone(),
        use_proxy: config.server.use_proxy,
    })); //.layer(cors);
    let app = app.fallback(handler_404);

    // Create index
    db.create_ips_index().await;

    let addr = format!("{}:{}", config.server.address, config.server.port);
    info!("listening on {}", addr);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        // Don't forget to add `ConnectInfo` if you aren't behind a proxy
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
    Ok(())
}
