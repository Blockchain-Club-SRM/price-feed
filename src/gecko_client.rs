use reqwest::Client;

pub struct GeckoClient {
    http_client : Client,
    url: String,
}

impl GeckoClient {
    pub fn new(
        url:String,
        timeout: std::time::Duration,
    )->Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            url,
        }
    }
    pub async fn get_request(&self, request: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}", self.url, request);
        let response = self.http_client
        .get(format!("{}",&url))
        .send()
        .await?
        .error_for_status()?;
        Ok(response)
    }
}