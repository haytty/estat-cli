use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::Requester;
use crate::model::term::Root;
use crate::service::create_json_file_service;

#[derive(Parser)]
pub struct TermArgs {
    #[arg(long, help = "lang: JP/EN")]
    lang: Option<String>,

    #[arg(long, help = "indicator_code")]
    indicator_code: Option<String>,

    #[arg(long, help = "category")]
    category: Option<String>,

    #[arg(long, help = "stat_code")]
    stat_code: Option<String>,

    #[arg(long, help = "search_term_word: part_match")]
    search_term_word: Option<String>,

    // #[arg(long, help = "callback")]
    // callback: String,
    //

    #[arg(long, help = "modified_from: yyyymmdd")]
    modified_from: Option<String>,

    #[arg(long, help = "modified_to: yyyymmdd")]
    modified_to: Option<String>,
}

const TERM_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getTermInfo";

impl Requester for TermArgs {
    fn to_url(&self) -> Result<Url> {
        let mut url = Url::parse(TERM_URL)?;

        let mut add_param = |url: &mut Url, key: &str, value: Option<&String>| {
            if let Some(val) = value {
                url.query_pairs_mut().append_pair(key, val);
            }
        };

        add_param(&mut url, "Lang", self.lang.as_ref());
        add_param(&mut url, "IndicatorCode", self.indicator_code.as_ref());
        add_param(&mut url, "Category", self.category.as_ref());
        add_param(&mut url, "StatCode", self.stat_code.as_ref());
        add_param(&mut url, "SearchTermWord", self.search_term_word.as_ref());
        add_param(&mut url, "ModifiedFrom", self.modified_from.as_ref());
        add_param(&mut url, "ModifiedTo", self.modified_to.as_ref());

        println!("{}", url);
        Ok(url)
    }
}

pub async fn handle(args: TermArgs) -> Result<()> {
    let result = create_json_file_service::call::<_, _, Root>(args, "/tmp/term.json").await?;
    Ok(result)
}
