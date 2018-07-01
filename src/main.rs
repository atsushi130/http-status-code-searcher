
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate rustc_serialize;

use std::env;

mod http_status_code_searcher;
use http_status_code_searcher::HttpStatusCodeSearcher;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args[1..].to_vec().is_empty() {
        println!("not found arguments.");
        return;
    }

    let input = &args[1];

    if input == "list" {
        let code_list = HttpStatusCodeSearcher.find_all_code();
        code_list.into_iter().for_each(|http_status_code| {
            println!("{}", http_status_code);
        });
    } else {
        let maybe_http_status_code = HttpStatusCodeSearcher.search_by(&input);
        maybe_http_status_code.iter().for_each(|http_status_code| {
            println!("{} {}", http_status_code.code, http_status_code.title);
            println!("{}", http_status_code.detail);
        });
    }
}
