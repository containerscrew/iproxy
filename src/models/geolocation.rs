use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoLocation {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none", default= "None")]
    // id: Option<ObjectId>,
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
    query: String,
    // #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    // released: chrono::DateTime<Utc>,
}
