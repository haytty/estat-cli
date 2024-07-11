use anyhow::Result;
use tokio::io::{AsyncWriteExt};
use crate::lib::http::request::Requester;
use crate::lib::record::Recorder;
use crate::lib::writer::Writer;

pub async fn call<R, T, W>(requester: R, mut writer: W) -> Result<()>
where
    R: Requester,
    T: Recorder,
    W: Writer,
{
    let body = requester.request().await?
        .text().await?;

    let root: T = serde_json::from_str(&body)?;
    let json = root.to_record_json()?;

    writer.write_all(json.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}