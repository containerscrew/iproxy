use std::sync::Arc;
use actix_web::{App, http, HttpMessage, HttpRequest, HttpResponse, test::{call_and_read_body, call_and_read_body_json, init_service, TestRequest}, web, web::Bytes};
use actix_web::http::header::{CONTENT_TYPE, ContentType, HeaderValue};
use actix_web::middleware::Logger;
use crate::infrastructure::Db;
use crate::models::{GeoLocation, Ip};
use crate::routes::{delete_ip, get_ip, health, insert_ip, update_ip};
use actix_web::test::call_service;
use reqwest::Method;

use super::*;

#[actix_web::test]
async fn should_insert_ip() {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let db_endpoint = std::env::var("DB_ENDPOINT").unwrap_or_else(|_| "mongodb://admin:admin@localhost:27017/?maxPoolSize=20&w=majority".into());

    // Register client
    let db: Arc<dyn DbOps + Send + Sync> = Arc::new(Db::new(
        db_endpoint,
        "ipfinder".to_string(),
        "ips".to_string(),
    ).await.expect("Can't connect to database"));

    //db.create_ips_index().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(insert_ip)
    ).await;

    let ip = Ip {
        ip: "4.4.4.4".to_string(),
    };

    // Post method
    let req = test::TestRequest::post()
        .uri("/insert")
        .set_json(&ip)
        .insert_header(ContentType::json())
        .to_request();
    println!("{:?}", req);

    let response = call_service(&app, req).await;
    println!("{:?}", response.response());
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn should_get_ip() {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let db_endpoint = std::env::var("DB_ENDPOINT").unwrap_or_else(|_| "mongodb://admin:admin@localhost:27017/?maxPoolSize=20&w=majority".into());

    // Register client
    let db: Arc<dyn DbOps + Send + Sync> = Arc::new(Db::new(
        db_endpoint,
        "ipfinder".to_string(),
        "ips".to_string(),
    ).await.expect("Can't connect to database"));

    let ip = Ip {
        ip: "4.4.4.4".to_string(),
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_ip)
    ).await;

// Get method
    let ip_geolocation = r#"
    {
        "status": "success",
        "city": "Honolulu",
        "countryCode": "US",
        "regionName": "Hawaii",
        "region": "HI",
        "country": "United States",
        "zip": "96805",
        "lat": 21.3069,
        "lon": -157.858,
        "timezone": "Pacific/Honolulu",
        "isp": "Level 3 Communications, Inc.",
        "org": "Level 3",
        "as": "AS3356 Level 3 Parent, LLC",
        "query": "4.4.4.4"
    }
    "#;

    let req = TestRequest::get()
        .uri(&format!("/get/{}", &ip.ip))
        .to_request();

    let response: GeoLocation = call_and_read_body_json(&app, req).await;
    let data: GeoLocation = serde_json::from_str(ip_geolocation).expect("Can't serialize Geolocation json from raw string");
    assert_eq!(response, data);
}