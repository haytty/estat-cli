use std::collections::HashMap;
use std::path::{PathBuf};
use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::{append_url_params, join_multiple_value, Requester};
use crate::lib::writer::{initialize_writer};
use crate::model::data::Root;
use crate::service::request_to_api_service;

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

    #[arg(long, help = "output_dir: output_dir_path")]
    output_dir: Option<String>,
}

const INDICATOR_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getIndicatorInfo";

impl Requester for DataArgs {
    fn to_url(&self) -> Result<Url> {
        let url = Url::parse(INDICATOR_URL)?;

        let indicator_codes = join_multiple_value(Some(self.indicator_code.as_ref()));
        let region_codes = join_multiple_value(self.region_code.as_ref());

        let map: HashMap<_, _> = vec![
            ("Lang", self.lang.as_ref()),
            ("IndicatorCode", indicator_codes.as_ref()),
            ("RegionCode", region_codes.as_ref()),
            ("ParentRegionCode", self.parent_region_code.as_ref()),
            ("RegionLevel", self.region_level.as_ref()),
            ("Time", self.time.as_ref()),
            ("TimeFrom", self.time_from.as_ref()),
            ("TimeTo", self.time_to.as_ref()),
            ("Cycle", self.cycle.as_ref()),
            ("RegionalRank", self.regional_rank.as_ref()),
            ("IsSeasonalAdjustment", self.is_seasonal_adjustment.as_ref()),
            ("StatName", self.stat_name.as_ref()),
            ("ValueCondition", self.value_condition.as_ref()),
            ("MetaGetFlg", self.meta_get_flg.as_ref()),
            ("SectionHeaderFlg", self.section_header_flg.as_ref()),
            ("ModifiedFrom", self.modified_from.as_ref()),
            ("ModifiedTo", self.modified_to.as_ref()),
        ].into_iter().collect();

        let url = append_url_params(url, &map);

        Ok(url)
    }
}

fn create_file_path(output_dir: &String, codes: &Vec<String>) -> PathBuf {
    let path_buf = PathBuf::from(output_dir);
    path_buf.join(format!("data_{}.json", codes.join("_")))
}

pub async fn handle(args: DataArgs) -> Result<()> {
    let path = match &args.output_dir {
        Some(o) => {
            Some(create_file_path(o, &args.indicator_code))
        }
        None => None,
    };

    let writer = initialize_writer(path).await?;
    let result = request_to_api_service::call::<_, Root, _>(args, writer).await?;

    Ok(result)
}
