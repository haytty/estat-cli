use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::Requester;
use crate::model::stat::Root;
use crate::service::create_json_file_service;

#[derive(Parser)]
pub struct StatArgs {
    #[arg(long, help = "lang: JP/EN")]
    lang: Option<String>,

    #[arg(long, help = "indicator_code: multiple code ok")]
    indicator_code: Option<Vec<String>>,

    #[arg(long, help = "stat_code")]
    stat_code: Option<String>,

    #[arg(long, help = "search_survey_word: part_match")]
    search_survey_word: Option<String>,

    // #[arg(long, help = "callback")]
    // callback: String,
    //

    #[arg(long, help = "modified_from: yyyymmdd")]
    modified_from: Option<String>,

    #[arg(long, help = "modified_to: yyyymmdd")]
    modified_to: Option<String>,
}

const STAT_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getStatInfo";

impl Requester for StatArgs {
    fn to_url(&self) -> Result<Url> {
        let mut url = Url::parse(STAT_URL)?;

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
        add_params(&mut url, "IndicatorCode", self.indicator_code.as_ref());
        add_param(&mut url, "StatCode", self.stat_code.as_ref());
        add_param(&mut url, "SearchSurveyWord", self.search_survey_word.as_ref());
        add_param(&mut url, "ModifiedFrom", self.modified_from.as_ref());
        add_param(&mut url, "ModifiedTo", self.modified_to.as_ref());

        println!("{}", url);
        Ok(url)
    }
}

pub async fn handle(args: StatArgs) -> Result<()> {
    let result = create_json_file_service::call::<_, _, Root>(args, "/tmp/stat.json").await?;
    Ok(result)
}
