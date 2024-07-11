use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Recorder: Serialize + DeserializeOwned {
    fn to_record_json(&self) -> Result<String>;
}

impl Recorder for serde_json::Value {
    fn to_record_json(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }
}
