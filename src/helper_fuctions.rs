use reqwest::{Client, Response};

pub async fn fetch_data(client: &Client) -> Result<Response, reqwest::Error> {
    client.get("https://test.com/api/data").send().await
}
