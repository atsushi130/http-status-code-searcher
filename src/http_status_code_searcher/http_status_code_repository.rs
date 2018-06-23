
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use super::{ HttpStatusCode, HttpStatusCodes };
use rustc_serialize::json;//::Json;
use std::fs::File;
use std::io::Read;

pub struct HttpStatusCodeRepository;

impl HttpStatusCodeRepository {

    pub fn find_all(&self) -> Vec<HttpStatusCode> {

        let dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/http_status_code.json", dir);
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let http_status_code: HttpStatusCodes = json::decode(&data).unwrap();
        http_status_code.values
    }
}