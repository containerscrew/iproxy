use serde::{Deserialize, Serialize};


// pub type DB = Arc<Mutex<Vec<Ip>>>;
//
// pub fn todo_db() -> DB {
//     Arc::new(Mutex::new(Vec::new()))
// }

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GeoLocation {
    status: String,
    city: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(rename = "regionName")]
    region_name: String,
    region: String,
    country: String,
    zip: String,
    lat: f64,
    lon: f64,
    timezone: String,
    isp: String,
    org: String,
    #[serde(rename = "as")]
    ass: String,
    //#[serde(rename(serialize = "ip", deserialize = "query"))]
    query: String,
}