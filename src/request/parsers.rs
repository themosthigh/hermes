use curl_parser::ParsedRequest;
use http_rest_file::Parser;
use std::str::FromStr;

use crate::request::{RequestHeader, RequestState};

impl RequestState {
    pub fn from_curl(text: &str) -> Result<Self, String> {
        match ParsedRequest::from_str(text.trim()) {
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

impl RequestState {
    pub fn from_http(input_string: String) -> Result<Self, String> {
        let result = Parser::parse(&input_string, true);
        let req = &result.requests[0];

        Ok(RequestState {
            url: req.get_url(),
            method: req.request_line.method.get_or_default().to_string(),
            headers: req
                .headers
                .iter()
                .map(|header| RequestHeader {
                    name: header.key.clone(),
                    value: header.value.clone(),
                    enabled: true,
                })
                .collect(),
            body: req.body.to_string(),
        })
    }
}
