
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct HttpStatusCodes {
    pub values: Vec<HttpStatusCode>
}

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct HttpStatusCode {
    pub code: String,
    pub title: String,
    pub detail: String
}
