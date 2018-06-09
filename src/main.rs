
mod http_status_code_searcher;
use http_status_code_searcher::HttpStatusCodeSearcher;

fn main() {
    let http_status_code = HttpStatusCodeSearcher.search_by("404");
    println!("code: {}", http_status_code.code);
}
