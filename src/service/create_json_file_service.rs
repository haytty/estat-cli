use std::path::Path;
use tokio::fs;
use anyhow::Result;
use serde::de::DeserializeOwned;
use crate::lib::http::request::Requester;
use crate::lib::record::Recorder;

pub async fn call<R, P, T>(requester: R, path: P) -> Result<()>
where
    R: Requester,
    P: AsRef<Path>,
    T: Recorder + DeserializeOwned,
{
    let body = requester.request().await?
        .text().await?;

    let root: T = serde_json::from_str(&body)?;
    let json = root.to_record_json()?;

    fs::write(path, json).await?;

    Ok(())
}