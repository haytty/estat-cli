use serde::{Deserialize, Serialize};
use serde_json::{Value};
use crate::lib::record::Recorder;
use anyhow::{Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    #[serde(flatten)]
    data: Value,
}


impl Recorder for Root {
    fn to_record_json(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(&self.data)?;
        Ok(json)
    }
}
