use reqwest::{Client, Method};

use serde_json;
use serde_json::json;

use crate::request::RequestState;

pub async fn send_request(request: RequestState) -> Result<String, String> {
    let client = Client::new();

    // println!("Request body: {}", request.body);

    let response = client
        .request(
            Method::from_bytes(request.method.as_bytes()).unwrap(),
            request.url,
        )
        .header("Content-Type", "application/json")
        .body(request.body)
        .send()
        .await;

    match response {
        Ok(response) => {
            // Get value from state
            let text = response.text().await.unwrap();

            // Get JSON value
            let json_value: serde_json::Value =
                serde_json::from_str(&text).unwrap_or_else(|error| {
                    json!({
                        "hermes_error": error.to_string(),
                        "message": "Failed to parse JSON value",
                        "text": text
                    })
                });

            // format JSON
            let formatted = serde_json::to_string_pretty(&json_value).unwrap_or_else(|error| {
                format!(
                    "Failed to format JSON\nError: {}\nText: {}",
                    error.to_string(),
                    text
                )
            });

            return Ok(formatted);
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }
}
