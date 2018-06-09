
extern crate rustc_serialize;

mod http_status_code_searcher;
use http_status_code_searcher::HttpStatusCodeSearcher;

fn main() {
    let http_status_code = HttpStatusCodeSearcher.search_by("200");
    http_status_code.iter().for_each(|http_status_code| {
        println!("{} {}", http_status_code.code, http_status_code.title);
        println!("{}", http_status_code.detail);
    });
}
