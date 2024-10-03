use crate::{models::GeoLocation, utils::serialize_geolocation_data};
use reqwest::Error;

const IP_API_ENDPOINT: &str = "http://ip-api.com/json/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";

pub async fn get_geolocation(info: &String) -> Result<GeoLocation, Error> {
    let client = reqwest::Client::new()
        .get(format!("{}{}", IP_API_ENDPOINT, info.to_string()))
        .header("User-Agent", USER_AGENT);

    let response = client.send().await?;

    // TODO: manage errors in request

    let response_text = response.text().await?;

    Ok(serialize_geolocation_data(&response_text))
}
