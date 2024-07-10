use anyhow::{Result};
use reqwest::{Response};
use url::{Url};

#[allow(async_fn_in_trait)]
pub trait Requester {
    fn to_url(&self) -> Result<Url>;

    async fn request(&self) -> Result<Response> {
        let url = self.to_url()?;
        let client = reqwest::Client::new();

        let result = client.get(url).send().await?;
        Ok(result)
    }
}