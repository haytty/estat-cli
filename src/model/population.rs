use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Deserialize, Clone)]
pub struct Population {
    #[serde(rename = "@indicator")]
    indicator: String,

    #[serde(rename = "@unit")]
    unit: String,

    #[serde(rename = "@stat")]
    stat: String,

    #[serde(rename = "@regionCode")]
    region_code: String,

    #[serde(rename = "@time")]
    time: String,

    #[serde(rename = "@cycle")]
    cycle: String,

    #[serde(rename = "@regionRank")]
    region_rank: String,

    #[serde(rename = "@isSeasonal")]
    is_seasonal: String,

    #[serde(rename = "@isProvisional")]
    is_provisional: String,

    #[serde(rename = "$")]
    value: String,
}

// 手動でシリアライズを実装(シリアライズの時にrenameの影響を受けたくないため)
impl Serialize for Population {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Population", 11)?;
        state.serialize_field("indicator", &self.indicator)?;
        state.serialize_field("unit", &self.unit)?;
        state.serialize_field("stat", &self.stat)?;
        state.serialize_field("region_code", &self.region_code)?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("cycle", &self.cycle)?;
        state.serialize_field("region_rank", &self.region_rank)?;
        state.serialize_field("is_seasonal", &self.is_seasonal)?;
        state.serialize_field("is_provisional", &self.is_provisional)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}


#[derive(Debug, Deserialize)]
struct DataObj {
    #[serde(rename = "VALUE")]
    population: Population,
}

#[derive(Debug, Deserialize)]
struct DataInf {
    #[serde(rename = "DATA_OBJ")]
    data_obj: Vec<DataObj>,
}

#[derive(Debug, Deserialize)]
struct StatisticalData {
    #[serde(rename = "DATA_INF")]
    data_inf: DataInf,
}

#[derive(Debug, Deserialize)]
pub struct GetStats {
    #[serde(rename = "STATISTICAL_DATA")]
    statistical_data: StatisticalData,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    #[serde(rename = "GET_STATS")]
    get_stats: GetStats,
}

impl Root {
    pub fn get_populations(&self) -> Vec<Population> {
        self.get_stats
            .statistical_data
            .data_inf
            .data_obj
            .iter()
            .map(|obj| obj.population.clone())
            .collect()
    }
}