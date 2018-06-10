
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

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