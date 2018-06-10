
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

    let http_status_code = &args[1];

    let maybe_http_status_code = HttpStatusCodeSearcher.search_by(&http_status_code);
    maybe_http_status_code.iter().for_each(|http_status_code| {
        println!("{} {}", http_status_code.code, http_status_code.title);
        println!("{}", http_status_code.detail);
    });
}
