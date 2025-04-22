use reqwest;

pub struct HttpConnection {}

impl HttpConnection {
    pub fn make_client(&self) -> reqwest::Client {
        reqwest::Client::new()
    }
}
