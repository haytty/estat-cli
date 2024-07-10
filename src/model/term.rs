use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::lib::record::Recorder;
use anyhow::{anyhow, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "GET_META_TERM_INFO")]
    get_meta_term_info: GetMetaTermInfo,
}

impl Root {
    fn get_terms(&self) -> Option<Vec<Term>> {
        let terms: Vec<Term> =
            self.get_meta_term_info.metadata_inf.clone()?
                .class_inf
                .class_obj.class;

        Some(terms)
    }
}

impl Recorder for Root {
    fn to_record_json(&self) -> Result<String> {
        let terms = self.get_terms().ok_or(anyhow!("Record Not Found Error."))?;
        let json = serde_json::to_string_pretty(&terms)?;
        Ok(json)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GetMetaTermInfo {
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

#[derive(Debug, Deserialize, Clone)]
struct ClassObj {
    #[serde(rename = "CLASS")]
    class: Vec<Term>,
}

impl Serialize for ClassObj {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("class_obj", 1)?;
        state.serialize_field("class", &self.class)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Term {
    #[serde(rename = "@category")]
    category: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@code")]
    code: String,
    #[serde(rename = "@detail")]
    detail: String,
    #[serde(rename = "@linkURL")]
    link_url: Option<String>,
}

impl Serialize for Term {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("term", 5)?;
        state.serialize_field("category", &self.category)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("detail", &self.detail)?;
        state.serialize_field("link_url", &self.link_url)?;
        state.end()
    }
}
