use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::lib::record::Recorder;
use anyhow::{anyhow, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    #[serde(rename = "GET_META_REGION_INF")]
    get_meta_region_inf: GetMetaRegionInf,
}

impl Root {
    fn get_regions(&self) -> Option<Vec<Region>> {
        let regions: Vec<Region> =
            self.get_meta_region_inf.metadata_inf.clone()?
                .class_inf
                .class_obj.clone();

        Some(regions)
    }
}

impl Recorder for Root {
    fn to_record_json(&self) -> Result<String> {
        let regions = self.get_regions().ok_or(anyhow!("Record Not Found Error."))?;
        let json = serde_json::to_string_pretty(&regions)?;
        Ok(json)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GetMetaRegionInf {
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
    class_obj: Vec<Region>,
}

#[derive(Debug, Deserialize, Clone)]
struct Region {
    #[serde(rename = "@parentRegionCode")]
    parent_region_code: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@hiragana")]
    hiragana: String,
    #[serde(rename = "CLASS")]
    class: Vec<RegionClass>,
}

impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("class_obj", 4)?;
        state.serialize_field("parent_region_code", &self.parent_region_code)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("hiragana", &self.hiragana)?;
        state.serialize_field("class", &self.class)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct RegionClass {
    #[serde(rename = "@regionCode")]
    region_code: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@level")]
    level: String,
    #[serde(rename = "@hiragana")]
    hiragana: String,
    #[serde(rename = "@fromDate")]
    from_date: String,
    #[serde(rename = "@toDate")]
    to_date: String,
}

impl Serialize for RegionClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("region_class", 6)?;
        state.serialize_field("region_code", &self.region_code)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("level", &self.level)?;
        state.serialize_field("hiragana", &self.hiragana)?;
        state.serialize_field("from_date", &self.from_date)?;
        state.serialize_field("to_date", &self.to_date)?;
        state.end()
    }
}
