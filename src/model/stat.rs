use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::lib::record::Recorder;
use anyhow::{anyhow, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "GET_META_STAT_INFO")]
    get_meta_stat_info: GetMetaStatInfo,
}

impl Root {
    fn get_statistics(&self) -> Option<Vec<Stat>> {
        let stats: Vec<Stat> =
            self.get_meta_stat_info.metadata_inf.clone()?
                .class_inf
                .class_obj.class;

        Some(stats)
    }
}

impl Recorder for Root {
    fn to_record_json(&self) -> Result<String> {
        let statistics = self.get_statistics().ok_or(anyhow!("Record Not Found Error."))?;
        let json = serde_json::to_string_pretty(&statistics)?;
        Ok(json)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GetMetaStatInfo {
    #[serde(rename = "RESULT")]
    result: ResultInfo,
    #[serde(rename = "PARAMETER")]
    parameter: ParameterInfo,
    #[serde(rename = "METADATA_INF")]
    metadata_inf: Option<MetadataInf>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResultInfo {
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "errorMsg")]
    error_msg: String,
    #[serde(rename = "date")]
    date: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ParameterInfo {
    #[serde(rename = "lang")]
    lang: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct MetadataInf {
    #[serde(rename = "CLASS_INF")]
    class_inf: ClassInf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClassInf {
    #[serde(rename = "CLASS_OBJ")]
    class_obj: ClassObj,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClassObj {
    #[serde(rename = "CLASS")]
    class: Vec<Stat>,
}

#[derive(Debug, Deserialize, Clone)]
struct Stat {
    #[serde(rename = "@code")]
    code: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@agency")]
    agency: String,
    #[serde(rename = "@kind")]
    kind: String,
    #[serde(rename = "@summary")]
    summary: Option<String>,
    #[serde(rename = "@linkUrl")]
    link_url: Option<String>,
}

impl Serialize for Stat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("stat", 6)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("agency", &self.agency)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("summary", &self.summary)?;
        state.serialize_field("link_url", &self.link_url)?;
        state.end()
    }
}
