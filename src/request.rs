use std::str::FromStr;

use curl_parser::ParsedRequest;

pub const METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"];

#[derive(Debug, Clone)]
pub struct RequestHeader {
    pub name: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct RequestState {
    pub url: String,
    pub method: String,
    pub headers: Vec<RequestHeader>,
    pub body: String,
}

impl Default for RequestState {
    fn default() -> Self {
        Self {
            url: String::from("https://api.nasa.gov/planetary/apod"),
            method: String::from("GET"),
            headers: vec![
                // JSON by default
                RequestHeader {
                    name: String::from("Content-Type"),
                    value: String::from("application/json"),
                    enabled: true,
                },
                RequestHeader {
                    name: String::from("Accept"),
                    value: String::from("application/json"),
                    enabled: true,
                },
            ],
            body: String::new(),
        }
    }
}

impl RequestState {
    pub fn from_curl(text: &str) -> Result<Self, String> {
        println!("Input {text}");
        match ParsedRequest::from_str(text) {
            Ok(parsed_request) => Ok(Self {
                url: parsed_request.url.to_string(),
                method: parsed_request.method.to_string(),
                headers: parsed_request
                    .headers
                    .iter()
                    .map(|header| RequestHeader {
                        name: header.0.to_string(),
                        value: header.1.to_str().unwrap().to_string(),
                        enabled: true,
                    })
                    .collect(),
                body: parsed_request.body.join("\n"),
            }),

            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResponseState {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Default for ResponseState {
    fn default() -> Self {
        Self {
            status_code: 0,
            headers: Vec::new(),
            body: String::new(),
        }
    }
}
