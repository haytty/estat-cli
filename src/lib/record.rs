use anyhow::Result;
use serde::de::DeserializeOwned;

pub trait Recorder: DeserializeOwned {
    fn to_record_json(&self) -> Result<String>;
}