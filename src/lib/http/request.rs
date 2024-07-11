use std::collections::HashMap;
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

pub fn append_url_params(mut url: Url, params: &HashMap<&str, Option<&String>>) -> Url {
    for (key, value) in params {
        if let Some(val) = value {
            url.query_pairs_mut().append_pair(key, val);
        }
    }

    url
}

pub fn join_multiple_value(values: Option<&Vec<String>>) -> Option<String> {
    values.map(|vals| vals.join(","))
}

// pub fn join_multiple_value(values: &Option<Vec<String>>) -> Option<String> {
//     match values {
//         Some(vals) => Some(vals.join(",")),
//         None => None
//     }
// }