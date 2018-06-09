
use super::HttpStatusCode;

pub struct HttpStatusCodeSearcher;

impl HttpStatusCodeSearcher {

    pub fn search_by(&self, code: &str) -> HttpStatusCode {
        HttpStatusCode::new(code,  "", "")
    }
}