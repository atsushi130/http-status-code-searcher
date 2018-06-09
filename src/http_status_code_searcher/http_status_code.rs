
pub struct HttpStatusCode {
    pub code: String,
    pub error: String,
    pub message: String
}

impl HttpStatusCode {
    pub fn new(code: &str, error: &str, message: &str) -> HttpStatusCode {
        HttpStatusCode {
            code: code.to_string(),
            error: error.to_string(),
            message: message.to_string()
        }
    }
}