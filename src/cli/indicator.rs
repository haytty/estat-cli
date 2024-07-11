use std::collections::HashMap;
use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::{append_url_params, join_multiple_value, Requester};
use crate::lib::path::create_file_path;
use crate::lib::writer::initialize_writer;
use crate::model::indicator::Root;
use crate::service::create_json_file_service;

#[derive(Parser)]
pub struct IndicatorArgs {
    #[arg(long, help = "lang: JP/EN")]
    lang: Option<String>,

    #[arg(long, help = "indicator_code: multiple code ok")]
    indicator_code: Option<Vec<String>>,

    #[arg(long, help = "category")]
    category: Option<String>,

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

    #[arg(long, help = "stat_code")]
    stat_code: Option<String>,

    #[arg(long, help = "stat_name: part_match")]
    stat_name: Option<String>,

    #[arg(long, help = "search_indicator_word: part_match")]
    search_indicator_word: Option<String>,

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

const INDICATOR_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getIndicatorInfo";

impl Requester for IndicatorArgs {
    fn to_url(&self) -> Result<Url> {
        let url = Url::parse(INDICATOR_URL)?;

        let indicator_codes = join_multiple_value(self.indicator_code.as_ref());

        let map: HashMap<_, _> = vec![
            ("Lang", self.lang.as_ref()),
            ("IndicatorCode", indicator_codes.as_ref()),
            ("Category", self.category.as_ref()),
            ("Time", self.time.as_ref()),
            ("TimeFrom", self.time_from.as_ref()),
            ("TimeTo", self.time_to.as_ref()),
            ("Cycle", self.cycle.as_ref()),
            ("RegionalRank", self.regional_rank.as_ref()),
            ("IsSeasonalAdjustment", self.is_seasonal_adjustment.as_ref()),
            ("StatCode", self.stat_code.as_ref()),
            ("StatName", self.stat_name.as_ref()),
            ("SearchIndicatorWord", self.search_indicator_word.as_ref()),
            ("ModifiedFrom", self.modified_from.as_ref()),
            ("ModifiedTo", self.modified_to.as_ref()),
        ].into_iter().collect();

        let url = append_url_params(url, &map);

        Ok(url)
    }
}

const BASE_FILE_NAME: &str = "indicator.json";

pub async fn handle(args: IndicatorArgs) -> Result<()> {
    let path = args.output_dir.as_ref().map(|dir| create_file_path(&dir, BASE_FILE_NAME));

    let writer = initialize_writer(path).await?;

    let result = match args.pretty {
        true => create_json_file_service::call::<_, Root, _>(args, writer).await?,
        _ => create_json_file_service::call::<_, serde_json::Value, _>(args, writer).await?,
    };

    Ok(result)
}
