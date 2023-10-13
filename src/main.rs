mod infrastructure;
mod routes;
mod models;

use std::env;
use std::string::ToString;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;
use log::{info};
use crate::infrastructure::{Db};
use crate::routes::{delete_ip, get_ip, health, insert_ip};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Init default logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting ipfinder...");

    // Load env variables with mongodb config
    let db_endpoint : String= env::var("DB_ENDPOINT").expect("You must set the DB_ENDPOINT environment var!");
    let db_name: String = env::var("DB_NAME").expect("You must set the DB_NAME environment var!");
    let collection_name: String = env::var("COLLECTION_NAME").expect("You must set the COLLECTION_NAME environment var!");

    // Register client
    let db = Db::new(db_endpoint, db_name, collection_name).await.expect("Can't connect to database");

    // Start the API
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1/ipfinder")
                .app_data(web::Data::new(db.clone()))
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(health)
                .service(insert_ip)
                .service(get_ip)
                .service(delete_ip)
        )
    }).workers(2)
        .bind(("127.0.0.1", 8081)).expect("Unable to bind address!")
        .shutdown_timeout(30)
        .run()
        .await
}