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
