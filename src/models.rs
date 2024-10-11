use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GeoLocation {
    status: String,
    city: String,
    #[serde(rename(serialize = "country_code", deserialize = "countryCode"))]
    country_code: Option<String>, // Changed to Option
    #[serde(rename(serialize = "region_name", deserialize = "regionName"))]
    region_name: Option<String>, // Changed to Option
    region: String,
    country: String,
    zip: String,
    lat: f64,
    lon: f64,
    timezone: String,
    isp: String,
    org: String,
    #[serde(rename(serialize = "as", deserialize = "as"))]
    as_field: Option<String>, // Adjusted field name for clarity
    #[serde(rename(serialize = "ip", deserialize = "query"))]
    ip: Option<String>,
}
