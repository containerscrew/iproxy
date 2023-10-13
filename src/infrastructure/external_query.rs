use reqwest::Error;
use crate::models::GeoLocation;

const IP_API_ENDPOINT: &str = "http://ip-api.com/json/";

fn serialize_json(response: &String) -> GeoLocation {
    let location: GeoLocation = serde_json::from_str(&response).expect("Can't serialize data");
    location
}

pub(crate) async fn get_geolocation(info: &String) -> Result<GeoLocation, Error> {
    let client = reqwest::Client::new()
        .get(format!("{}{}",IP_API_ENDPOINT, info.to_string()))
        .header("User-Agent", "Rust_Geolocator/1.0")
        ;

    let response = client.send().await?;
    let response_text = response.text().await?;

    Ok(serialize_json(&response_text))
}