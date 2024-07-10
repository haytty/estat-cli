use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::Requester;
use crate::model::region::Root;
use crate::service::create_json_file_service;

#[derive(Parser)]
pub struct RegionArgs {
    #[arg(long, help = "lang: JP/EN")]
    lang: Option<String>,

    #[arg(long, help = "region_code: multiple code ok")]
    region_code: Option<Vec<String>>,

    #[arg(long, help = "parent_region_code")]
    parent_region_code: Option<String>,

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

    #[arg(long, help = "region_level: 1 - 13")]
    region_level: Option<String>,

    #[arg(long, help = "search_region_word: part_match")]
    search_region_word: Option<String>,

    // #[arg(long, help = "callback")]
    // callback: String,
    //

    #[arg(long, help = "modified_from: yyyymmdd")]
    modified_from: Option<String>,

    #[arg(long, help = "modified_to: yyyymmdd")]
    modified_to: Option<String>,
}

const REGION_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getRegionInfo";

impl Requester for RegionArgs {
    fn to_url(&self) -> Result<Url> {
        let mut url = Url::parse(REGION_URL)?;

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
        add_params(&mut url, "RegionCode", self.region_code.as_ref());
        add_param(&mut url, "ParentRegionCode", self.parent_region_code.as_ref());
        add_param(&mut url, "Time", self.time.as_ref());
        add_param(&mut url, "TimeFrom", self.time_from.as_ref());
        add_param(&mut url, "TimeTo", self.time_to.as_ref());
        add_param(&mut url, "RegionLevel", self.region_level.as_ref());
        add_param(&mut url, "SearchRegionWord", self.search_region_word.as_ref());
        add_param(&mut url, "ModifiedFrom", self.modified_from.as_ref());
        add_param(&mut url, "ModifiedTo", self.modified_to.as_ref());

        println!("{}", url);
        Ok(url)
    }
}

pub async fn handle(args: RegionArgs) -> Result<()> {
    let result = create_json_file_service::call::<_, _, Root>(args, "/tmp/region.json").await?;
    Ok(result)
}
