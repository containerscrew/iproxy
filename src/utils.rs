use crate::models::GeoLocation;

pub fn serialize_geolocation_data(response: &str) -> GeoLocation {
    let location: GeoLocation =
        serde_json::from_str(response).expect("Can't serialize geolocation data");
    location
}
