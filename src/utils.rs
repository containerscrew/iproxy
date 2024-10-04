use crate::models::GeoLocation;
use serde_json::Error;

pub fn serialize_geolocation_data(response: &str) -> Result<GeoLocation, Error> {
    let location: GeoLocation = serde_json::from_str(response)?;
    Ok(location)
}
