use tracing::info;
use crate::config::Config;
use crate::handlers::handler_404;
use crate::logger::setup_logger;
use crate::router::create_router;

mod logger;
mod router;
mod handlers;
mod models;
mod external;
mod mongodb;
mod config;

#[tokio::main]
async fn main(){
    // Load configuration file
    let config = Config::from_file("config.toml");

    // Enable logging
    setup_logger(config.logging.log_level);
    info!("Hello iproxy");

    // TODO: control CORS

    // Run server
    let app = create_router();  //.layer(cors);
    let app = app.fallback(handler_404);


    let addr = format!("{}:{}", config.server.address, config.server.port);

    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}