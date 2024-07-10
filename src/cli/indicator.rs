use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::Requester;
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
}

const INDICATOR_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getIndicatorInfo";

impl Requester for IndicatorArgs {
    fn to_url(&self) -> Result<Url> {
        let mut url = Url::parse(INDICATOR_URL)?;

        let mut add_param = |url: &mut Url, key: &str, value: Option<&String>| {
            if let Some(val) = value {
                url.query_pairs_mut().append_pair(key, val);
            }
        };

        let mut add_params = |url: &mut Url, key: &str, values: Option<&Vec<String>>| {
            if let Some(vals) = values {
                url.query_pairs_mut().append_pair(key, &*vals.join(","));
            }
        };

        add_param(&mut url, "Lang", self.lang.as_ref());
        add_params(&mut url, "IndicatorCode", self.indicator_code.as_ref());
        add_param(&mut url, "Category", self.category.as_ref());
        add_param(&mut url, "Time", self.time.as_ref());
        add_param(&mut url, "TimeFrom", self.time_from.as_ref());
        add_param(&mut url, "TimeTo", self.time_to.as_ref());
        add_param(&mut url, "Cycle", self.cycle.as_ref());
        add_param(&mut url, "RegionalRank", self.regional_rank.as_ref());
        add_param(&mut url, "IsSeasonalAdjustment", self.is_seasonal_adjustment.as_ref());
        add_param(&mut url, "StatCode", self.stat_code.as_ref());
        add_param(&mut url, "StatName", self.stat_name.as_ref());
        add_param(&mut url, "SearchIndicatorWord", self.search_indicator_word.as_ref());
        add_param(&mut url, "ModifiedFrom", self.modified_from.as_ref());
        add_param(&mut url, "ModifiedTo", self.modified_to.as_ref());

        println!("{}", url);
        Ok(url)
    }
}

pub async fn handle(args: IndicatorArgs) -> Result<()> {
    let result = create_json_file_service::call::<_, _, Root>(args, "/tmp/indicator.json").await?;
    Ok(result)
}
