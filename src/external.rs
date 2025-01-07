use axum::Json;
use rand::seq::SliceRandom;
use reqwest::{Client, Error, Proxy, StatusCode};
use serde_json::Value;
use std::time::Duration;
use tracing::{error, trace, warn};

const IP_API_ENDPOINT: &str = "http://ip-api.com/json/";
const MAX_RETRIES: usize = 5; // Maximum number of retries if the request fails

const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:81.0) Gecko/20100101 Firefox/81.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1 Mobile/15E148 Safari/604.1",
];

const PROXIES: &[&str] = &[
    "http://34.81.160.132:80",     // Taiwan
    "http://34.87.84.105:80",      // Singapore
    "http://117.54.114.102:80",    // Indonesia
    "http://47.178.24.220:80",     // United States
    "http://160.86.242.23:8080",   // Japan
    "http://20.26.249.29:8080",    // United Kingdom
    "http://198.49.68.80:80",      // United States
    "http://154.64.226.138:80",    // Japan
    "http://89.213.0.29:80",       // Hong Kong
    "http://51.222.161.115:80",    // Canada
    "http://195.181.172.220:8080", // Netherlands
    "http://41.169.69.91:3128",    // South Africa
    "http://85.215.64.49:80",      // Germany
    "http://162.223.90.130:80",    // United States
    "http://23.247.136.245:80",    // Singapore
    "http://133.18.234.13:80",     // Japan
    "http://41.204.53.19:80",      // Ghana
    "http://41.204.53.30:80",      // Ghana
];

// Function to get a random User-Agent
fn get_random_user_agent() -> &'static str {
    USER_AGENTS.choose(&mut rand::thread_rng()).unwrap()
}

// Function to configure the reqwest client, optionally using a proxy
fn configure_client(use_proxy: bool) -> Result<Client, Error> {
    let mut client_builder = reqwest::Client::builder().timeout(Duration::from_secs(10)); // Set timeout of 10 seconds

    if use_proxy {
        // Randomly select a proxy from the list
        let proxy_url = PROXIES.choose(&mut rand::thread_rng()).unwrap();
        let proxy = Proxy::all(*proxy_url)?;
        client_builder = client_builder.proxy(proxy);

        trace!("Proxy enabled: using proxy {}", proxy_url);
    } else {
        trace!("Proxy disabled: connecting directly");
    }

    client_builder.build()
}

// Main function to perform the request, retrying if the request fails
pub async fn get_geolocation(info: &String, use_proxy: bool) -> Result<Json<Value>, Error> {
    let mut attempts = 0; // Number of attempts

    while attempts < MAX_RETRIES {
        attempts += 1;

        // Get random User-Agent
        let user_agent = get_random_user_agent();

        // Configure the client with or without proxy
        let client = match configure_client(use_proxy) {
            Ok(c) => c,
            Err(e) => {
                warn!("Error configuring client: {:?}", e);
                continue; // If client configuration fails, retry
            }
        };

        // Log the User-Agent being used
        trace!(
            "Attempting request using User-Agent: '{}' (Attempt {}/{})",
            user_agent,
            attempts,
            MAX_RETRIES
        );

        // Make the request
        let response = client
            .get(format!("{}{}", IP_API_ENDPOINT, info))
            .header("User-Agent", user_agent)
            .send()
            .await;

        match response {
            Ok(resp) => {
                // If the request is successful
                if resp.status().is_success() {
                    trace!("Request succeeded with status: {}", resp.status());
                    let response_json: Value = resp.json().await?;
                    return Ok(Json(response_json));
                } else {
                    warn!(
                        "Request failed with status: {} (Attempt {}/{})",
                        resp.status(),
                        attempts,
                        MAX_RETRIES
                    );
                    // If status code indicates too many requests, wait before retrying
                    if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                        warn!("Too many requests, retrying after a delay...");
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                }
            }
            Err(e) => {
                error!(
                    "Request error: {:?} (Attempt {}/{})",
                    e, attempts, MAX_RETRIES
                );
                // If a connection error occurs, retry
            }
        }
    }

    // Return a simple JSON error message after max retries
    Ok(Json(serde_json::json!({
        "error": "Max retries reached"
    })))
}
