use std::collections::HashMap;
use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::{append_url_params, Requester};
use crate::lib::path::create_file_path;
use crate::lib::writer::initialize_writer;
use crate::model::term::Root;
use crate::service::request_to_api_service;

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

    #[arg(long, help = "output_dir: output_dir_path")]
    output_dir: Option<String>,

    #[arg(long, help = "pretty: cleaning Unwanted chars")]
    pretty: bool,
}

const TERM_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getTermInfo";

impl Requester for TermArgs {
    fn to_url(&self) -> Result<Url> {
        let url = Url::parse(TERM_URL)?;

        let map: HashMap<_, _> = vec![
            ("Lang", self.lang.as_ref()),
            ("IndicatorCode", self.indicator_code.as_ref()),
            ("Category", self.category.as_ref()),
            ("StatCode", self.stat_code.as_ref()),
            ("SearchTermWord", self.search_term_word.as_ref()),
            ("ModifiedFrom", self.modified_from.as_ref()),
            ("ModifiedTo", self.modified_to.as_ref()),
        ].into_iter().collect();

        let url = append_url_params(url, &map);

        Ok(url)
    }
}

const BASE_FILE_NAME: &str = "term.json";

pub async fn handle(args: TermArgs) -> Result<()> {
    let path = args.output_dir
        .as_ref()
        .map(|dir| create_file_path(&dir, BASE_FILE_NAME));

    let writer = initialize_writer(path).await?;

    let result = match args.pretty {
        true => request_to_api_service::call::<_, Root, _>(args, writer).await?,
        _ => request_to_api_service::call::<_, serde_json::Value, _>(args, writer).await?,
    };

    Ok(result)
}
