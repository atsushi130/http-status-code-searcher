
use super::{ HttpStatusCode, HttpStatusCodeRepository };

pub struct HttpStatusCodeSearcher;

impl HttpStatusCodeSearcher {

    pub fn search_by(&self, code: &str) -> Option<HttpStatusCode> {
        HttpStatusCodeRepository.find_all()
            .into_iter()
            .find(|http_status_code| {
                http_status_code.code == code.to_string()
            })
    }
}