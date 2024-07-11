use std::collections::HashMap;
use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::{append_url_params, join_multiple_value, Requester};
use crate::lib::path::create_file_path;
use crate::lib::writer::initialize_writer;
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

    #[arg(long, help = "output_dir: output_dir_path")]
    output_dir: Option<String>,

    #[arg(long, help = "pretty: cleaning Unwanted chars")]
    pretty: bool,
}

const REGION_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getRegionInfo";

impl Requester for RegionArgs {
    fn to_url(&self) -> Result<Url> {
        let url = Url::parse(REGION_URL)?;

        let region_codes = join_multiple_value(self.region_code.as_ref());

        let map: HashMap<_, _> = vec![
            ("Lang", self.lang.as_ref()),
            ("RegionCode", region_codes.as_ref()),
            ("ParentRegionCode", self.parent_region_code.as_ref()),
            ("Time", self.time.as_ref()),
            ("TimeFrom", self.time_from.as_ref()),
            ("TimeTo", self.time_to.as_ref()),
            ("RegionLevel", self.region_level.as_ref()),
            ("SearchRegionWord", self.search_region_word.as_ref()),
            ("ModifiedFrom", self.modified_from.as_ref()),
            ("ModifiedTo", self.modified_to.as_ref()),
        ].into_iter().collect();

        let url = append_url_params(url, &map);

        Ok(url)
    }
}

const BASE_FILE_NAME: &str = "region.json";

pub async fn handle(args: RegionArgs) -> Result<()> {
    let path = args.output_dir.as_ref().map(|dir| create_file_path(&dir, BASE_FILE_NAME));

    let writer = initialize_writer(path).await?;

    let result = match args.pretty {
        true => create_json_file_service::call::<_, Root, _>(args, writer).await?,
        _ => create_json_file_service::call::<_, serde_json::Value, _>(args, writer).await?,
    };

    Ok(result)
}
