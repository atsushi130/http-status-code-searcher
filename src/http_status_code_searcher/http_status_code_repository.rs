
use super::{ HttpStatusCode, HttpStatusCodes };
use rustc_serialize::json;//::Json;
use std::fs::File;
use std::io::Read;

pub struct HttpStatusCodeRepository;

impl HttpStatusCodeRepository {

    pub fn find_all(&self) -> Vec<HttpStatusCode> {

        let mut file = File::open("http_status_code.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let http_status_code: HttpStatusCodes = json::decode(&data).unwrap();
        http_status_code.values
    }
}