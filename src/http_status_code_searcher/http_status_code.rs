
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
