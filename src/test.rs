use std::sync::Arc;
use actix_web::{App, test::{call_and_read_body, call_and_read_body_json, init_service, TestRequest}, web, web::Bytes};
use actix_web::middleware::Logger;
use crate::infrastructure::Db;
use crate::models::{GeoLocation, Ip};
use crate::routes::{delete_ip, get_ip, health, insert_ip, update_ip};

use super::*;
#[actix_web::test]
#[ignore = "requires MongoDB instance running"]
async fn test() {
    let db_endpoint = std::env::var("DB_ENDPOINT").unwrap_or_else(|_| "mongodb://admin:admin@localhost:27017/?maxPoolSize=20&w=majority".into());

    // Register client
    let db = Arc::new(Db::new(
        db_endpoint,
        "ipfinder".to_string(),
        "ips".to_string()
    ).await.expect("Can't connect to database"));

    db.create_ips_index().await;

    let app = init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(health)
            .service(insert_ip)
            .service(get_ip)
            .service(update_ip)
            .service(delete_ip)
    ).await;

    let ip = Ip{
        ip: "4.4.4.4".into(),
    };

    // Post method
    let req = TestRequest::post()
        .uri("/api/v1/ipfinder/insert")
        .set_form(&ip)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"ip saved"));

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
        .uri(&format!("/api/v1/ipfinder/{}", &ip.ip))
        .to_request();

    let response: GeoLocation = call_and_read_body_json(&app, req).await;
    let data: GeoLocation = serde_json::from_str(ip_geolocation).expect("Can't serialize Geolocation json from raw string");
    assert_eq!(response, data);
}
