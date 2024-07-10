use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::lib::record::Recorder;
use anyhow::{anyhow, Result};

#[derive(Debug, Deserialize, Clone)]
pub struct Indicator {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@code")]
    code: String,

    #[serde(rename = "annotations")]
    annotations: Option<Vec<Annotation>>,
    #[serde(rename = "details")]
    details: Option<Details>,
    #[serde(rename = "CLASS")]
    class: Option<Vec<Class>>,
}

// 手動でシリアライズを実装(シリアライズの時にrenameの影響を受けたくないため)
impl Serialize for Indicator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("indicator", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("code", &self.code)?;

        if let Some(annotations) = &self.annotations {
            state.serialize_field("annotations", annotations)?;
        } else {
            state.skip_field("annotations")?;
        }

        if let Some(details) = &self.details {
            state.serialize_field("details", details)?;
        } else {
            state.skip_field("details")?;
        }

        if let Some(class) = &self.class {
            state.serialize_field("class", class)?;
        } else {
            state.skip_field("class")?;
        }

        state.end()
    }
}


#[derive(Debug, Deserialize, Clone)]
struct Annotation {
    #[serde(rename = "@cycle")]
    cycle: String,
    #[serde(rename = "@regionalRank")]
    regional_rank: String,
    #[serde(rename = "@isSeasonal")]
    is_seasonal: String,
    #[serde(rename = "@annotation")]
    annotation: String,
}

impl Serialize for Annotation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("annotation", 4)?;
        state.serialize_field("cycle", &self.cycle)?;
        state.serialize_field("regional_rank", &self.regional_rank)?;
        state.serialize_field("is_seasonal", &self.is_seasonal)?;
        state.serialize_field("annotation", &self.annotation)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Details {
    #[serde(rename = "detail")]
    detail: Vec<Detail>,
}

impl Serialize for Details {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("details", 1)?;
        state.serialize_field("detail", &self.detail)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Detail {
    #[serde(rename = "@code")]
    code: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$")]
    value: String,
}

impl Serialize for Detail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("detail", 3)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Class {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@sname")]
    sname: Option<String>,

    #[serde(rename = "@fromDate")]
    from_date: String,

    #[serde(rename = "@toDate")]
    to_date: String,

    #[serde(rename = "cycle")]
    cycle: Cycle,

    #[serde(rename = "RegionalRank")]
    regional_rank: RegionalRank,

    #[serde(rename = "IsSeasonal")]
    is_seasonal: IsSeasonal,

    #[serde(rename = "@statName")]
    stat_name: String,

    #[serde(rename = "@unit")]
    unit: String,
}

impl Serialize for Class {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("class", 9)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("sname", &self.sname)?;
        state.serialize_field("from_date", &self.from_date)?;
        state.serialize_field("to_date", &self.to_date)?;
        state.serialize_field("cycle", &self.cycle)?;
        state.serialize_field("regional_rank", &self.regional_rank)?;
        state.serialize_field("is_seasonal", &self.is_seasonal)?;
        state.serialize_field("stat_name", &self.stat_name)?;
        state.serialize_field("unit", &self.unit)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Cycle {
    #[serde(rename = "@code")]
    code: String,

    #[serde(rename = "@name")]
    name: String,
}

impl Serialize for Cycle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("cycle", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("code", &self.code)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct RegionalRank {
    #[serde(rename = "@code")]
    code: String,

    #[serde(rename = "@name")]
    name: String,
}

impl Serialize for RegionalRank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("regional_rank", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("code", &self.code)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct IsSeasonal {
    #[serde(rename = "@code")]
    code: String,

    #[serde(rename = "@name")]
    name: String,
}

impl Serialize for IsSeasonal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("is_seasonal", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("code", &self.code)?;
        state.end()
    }
}

// IndicatorResponse

#[derive(Debug, Deserialize)]
pub struct Root {
    #[serde(rename = "GET_META_INDICATOR_INF")]
    get_meta_indicator_inf: GetMetaIndicatorInf,
}

impl Root {
    fn get_indicator(&self) -> Option<Vec<Indicator>> {
        let indicators: Vec<Indicator> =
            self.get_meta_indicator_inf.metadata_inf.clone()?
                .class_inf
                .class_obj.clone();

        Some(indicators)
    }
}

impl Recorder for Root {
    fn to_record_json(&self) -> Result<String> {
        let indicators = self.get_indicator().ok_or(anyhow!("Record Not Found Error."))?;
        let json = serde_json::to_string_pretty(&indicators)?;
        Ok(json)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GetMetaIndicatorInf {
    #[serde(rename = "RESULT")]
    result: ResultInfo,
    #[serde(rename = "PARAMETER")]
    parameter: Parameter,
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
struct Parameter {
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
    class_obj: Vec<Indicator>,
}
