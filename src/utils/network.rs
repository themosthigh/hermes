use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::request::{RequestState, ResponseState};

pub async fn send_request(request: RequestState) -> Result<ResponseState, String> {
    let client = Client::new();

    let mut header_map = HeaderMap::new();
    for header in request.headers {
        if header.enabled {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(header.name.as_bytes()),
                HeaderValue::from_str(header.value.as_str()),
            ) {
                header_map.insert(name, value);
            }
        }
    }

    let payload = client
        .request(
            Method::from_bytes(request.method.as_bytes()).unwrap(),
            request.url,
        )
        .headers(header_map)
        .body(request.body);

    let response = payload.send().await;

    match response {
        Ok(response) => {
            let response_state = ResponseState {
                status_code: response.status().as_u16(),
                headers: response
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                    .collect(),
                body: response.text().await.unwrap(),
            };
            return Ok(response_state);
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }
}

pub fn format_json(json: String) -> Result<String, String> {
    let json_value: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&json);

    match json_value {
        Ok(parsed_value) => {
            let formatted = serde_json::to_string_pretty(&parsed_value).unwrap_or_else(|error| {
                format!(
                    "Failed to format JSON\nError: {}\nText: {}",
                    error.to_string(),
                    json
                )
            });
            return Ok(formatted);
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }
}
