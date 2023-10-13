mod infrastructure;
mod routes;
mod models;
mod app;

use std::env;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;
use log::{info};
use crate::app::db_ops::DbOps;
use crate::infrastructure::{Db};
use crate::routes::{get_ip, health, insert_ip};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    get_env();

    info!("Starting ipfinder...");

    //Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let db = Arc::new(Db::new(
       client_uri,
       "ipfinder".to_string(),
       "ips".to_string()).await.expect("Can't connect to database"
    ));

    start_server(db).await
}

fn get_env() {
    env::set_var("RUST_LOG", "actix_web=debug");
    dotenv().ok();

    // Init default logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

fn start_server(db: Arc<dyn DbOps+Send+Sync>) -> Server {
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1/ipfinder")
                .app_data(web::Data::new(db.clone()))
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(health)
                .service(insert_ip)
                .service(get_ip)
        )
    }).workers(2)
        .bind(("127.0.0.1", 8081)).expect("Unable to bind address!")
        .shutdown_timeout(30)
        .run()
}