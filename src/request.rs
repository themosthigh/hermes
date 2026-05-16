pub const METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"];

#[derive(Debug, Clone)]
pub struct RequestState {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Default for RequestState {
    fn default() -> Self {
        Self {
            url: String::from("https://rickandmortyapi.com/api"),
            method: String::from("GET"),
            headers: Vec::new(),
            body: String::new(),
        }
    }
}
