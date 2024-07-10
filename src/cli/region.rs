use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::Requester;
use crate::model::indicator::Root;
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

        if let Some(lang) = &self.lang {
            url.query_pairs_mut().append_pair("Lang", lang);
        }

        if let Some(parent_region_code) = &self.parent_region_code {
            url.query_pairs_mut().append_pair("parent_region_code", parent_region_code);
        }

        Ok(url)
    }
}

pub async fn handle(args: RegionArgs) -> Result<()> {
    let result = create_json_file_service::call::<_, _, Root>(args, "/tmp/indicator.json").await?;
    Ok(result)
}
