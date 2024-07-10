use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::lib::record::Recorder;
use anyhow::{anyhow, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "GET_META_SOCIAL_INFO")]
    get_meta_social_info: GetMetaSocialInfo,
}

impl Root {
    fn get_social_events(&self) -> Option<Vec<SocialEvent>> {
        let social_events: Vec<SocialEvent> =
            self.get_meta_social_info.metadata_inf.clone()?
                .class_inf
                .class_obj.clone();

        Some(social_events)
    }
}

impl Recorder for Root {
    fn to_record_json(&self) -> Result<String> {
        let social_events = self.get_social_events().ok_or(anyhow!("Record Not Found Error."))?;
        let json = serde_json::to_string_pretty(&social_events)?;
        Ok(json)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GetMetaSocialInfo {
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
    #[serde(rename = "Lang")]
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
    class_obj: Vec<SocialEvent>,
}

#[derive(Debug, Deserialize, Clone)]
struct SocialEvent {
    #[serde(rename = "@code")]
    code: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@level")]
    level: String,
    #[serde(rename = "@fromTime")]
    from_time: String,
    #[serde(rename = "@toTime")]
    to_time: String,
    #[serde(rename = "CLASS")]
    class: Vec<EventClass>,
}

impl Serialize for SocialEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("social_event", 6)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("level", &self.level)?;
        state.serialize_field("from_time", &self.from_time)?;
        state.serialize_field("to_time", &self.to_time)?;
        state.serialize_field("class", &self.class)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct EventClass {
    #[serde(rename = "@code")]
    code: String,
    #[serde(rename = "@name")]
    name: String,
}

impl Serialize for EventClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("class", 2)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}
