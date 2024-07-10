use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::Requester;
use crate::model::data::Root;
use crate::service::create_json_file_service;

#[derive(Parser)]
pub struct DataArgs {
    #[arg(long, help = "lang: JP/EN")]
    lang: Option<String>,

    #[arg(long, help = "indicator_code: multiple code ok")]
    indicator_code: Vec<String>,

    #[arg(long, help = "region_code: multiple code ok")]
    region_code: Option<Vec<String>>,

    #[arg(long, help = "parent_region_code")]
    parent_region_code: Option<String>,

    #[arg(long, help = "region_level: 1 - 13")]
    region_level: Option<String>,

    #[arg(
        long, help = "time: Month<yyyymm00>, Quoter<yyyy1Q00>, Calendar<yyyyCY00>, Year<yyyyFY00>"
    )]
    time: Option<String>,

    #[arg(
        long, help = "time: Month<yyyymm00>, Quoter<yyyy1Q00>, Calendar<yyyyCY00>, Year<yyyyFY00>"
    )]
    time_from: Option<String>,

    #[arg(
        long, help = "time: Month<yyyymm00>, Quoter<yyyy1Q00>, Calendar<yyyyCY00>, Year<yyyyFY00>"
    )]
    time_to: Option<String>,

    #[arg(long, help = "cycle: 1 - 4")]
    cycle: Option<String>,

    #[arg(long, help = "regional_rank: 1 - 4")]
    regional_rank: Option<String>,

    #[arg(long, help = "is_seasonal_adjustment: 1 - 2")]
    is_seasonal_adjustment: Option<String>,

    #[arg(long, help = "stat_name: part_match")]
    stat_name: Option<String>,

    #[arg(long, help = "value_condition: part_match")]
    value_condition: Option<String>,

    #[arg(long, help = "meta_get_flg: part_match")]
    meta_get_flg: Option<String>,

    #[arg(long, help = "section_header_flg: part_match")]
    section_header_flg: Option<String>,

    // #[arg(long, help = "callback")]
    // callback: String,
    //

    #[arg(long, help = "modified_from: yyyymmdd")]
    modified_from: Option<String>,

    #[arg(long, help = "modified_to: yyyymmdd")]
    modified_to: Option<String>,
}

const INDICATOR_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getIndicatorInfo";

impl Requester for DataArgs {
    fn to_url(&self) -> Result<Url> {
        let mut url = Url::parse(INDICATOR_URL)?;

        let add_param = |url: &mut Url, key: &str, value: Option<&String>| {
            if let Some(val) = value {
                url.query_pairs_mut().append_pair(key, val);
            }
        };

        let add_params = |url: &mut Url, key: &str, values: Option<&Vec<String>>| {
            if let Some(vals) = values {
                url.query_pairs_mut().append_pair(key, &*vals.join(","));
            }
        };

        add_param(&mut url, "Lang", self.lang.as_ref());
        add_params(&mut url, "IndicatorCode", Some(self.indicator_code.as_ref()));
        add_params(&mut url, "RegionCode", self.region_code.as_ref());
        add_param(&mut url, "ParentRegionCode", self.parent_region_code.as_ref());
        add_param(&mut url, "RegionLevel", self.region_level.as_ref());
        add_param(&mut url, "Time", self.time.as_ref());
        add_param(&mut url, "TimeFrom", self.time_from.as_ref());
        add_param(&mut url, "TimeTo", self.time_to.as_ref());
        add_param(&mut url, "Cycle", self.cycle.as_ref());
        add_param(&mut url, "RegionalRank", self.regional_rank.as_ref());
        add_param(&mut url, "IsSeasonalAdjustment", self.is_seasonal_adjustment.as_ref());
        add_param(&mut url, "StatName", self.stat_name.as_ref());
        add_param(&mut url, "ValueCondition", self.value_condition.as_ref());
        add_param(&mut url, "MetaGetFlg", self.meta_get_flg.as_ref());
        add_param(&mut url, "SectionHeaderFlg", self.section_header_flg.as_ref());
        add_param(&mut url, "ModifiedFrom", self.modified_from.as_ref());
        add_param(&mut url, "ModifiedTo", self.modified_to.as_ref());

        println!("{}", url);
        Ok(url)
    }
}

pub async fn handle(args: DataArgs) -> Result<()> {
    let result = create_json_file_service::call::<_, _, Root>(args, "/tmp/data.json").await?;
    Ok(result)
}
