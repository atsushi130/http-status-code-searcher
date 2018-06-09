
pub struct HttpStatusCode {
    pub code: String,
    pub title: String,
    pub detail: String
}

impl HttpStatusCode {
    pub fn new(code: &str, title: &str, detail: &str) -> HttpStatusCode {
        HttpStatusCode {
            code: code.to_string(),
            title: title.to_string(),
            detail: detail.to_string()
        }
    }
}