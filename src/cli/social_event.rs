use std::collections::HashMap;
use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::{append_url_params, Requester};
use crate::lib::path::create_file_path;
use crate::lib::writer::initialize_writer;
use crate::model::social_event::Root;
use crate::service::create_json_file_service;

#[derive(Parser)]
pub struct SocialEventArgs {
    #[arg(long, help = "lang: JP/EN")]
    lang: Option<String>,

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

    #[arg(long, help = "social_event_level: 1 - 3")]
    social_event_level: Option<String>,

    #[arg(long, help = "category")]
    category: Option<String>,

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

const SOCIAL_EVENT_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getSocialEventInfo";

impl Requester for SocialEventArgs {
    fn to_url(&self) -> Result<Url> {
        let url = Url::parse(SOCIAL_EVENT_URL)?;

        let map: HashMap<_, _> = vec![
            ("Lang", self.lang.as_ref()),
            ("Time", self.time.as_ref()),
            ("TimeFrom", self.time_from.as_ref()),
            ("TimeTo", self.time_to.as_ref()),
            ("SocialEventLevel", self.social_event_level.as_ref()),
            ("Category", self.category.as_ref()),
            ("ModifiedFrom", self.modified_from.as_ref()),
            ("ModifiedTo", self.modified_to.as_ref()),
        ].into_iter().collect();

        let url = append_url_params(url, &map);

        Ok(url)
    }
}

const BASE_FILE_NAME: &str = "social_event.json";

pub async fn handle(args: SocialEventArgs) -> Result<()> {
    let path = args.output_dir.as_ref().map(|dir| create_file_path(&dir, BASE_FILE_NAME));

    let writer = initialize_writer(path).await?;

    let result = match args.pretty {
        true => create_json_file_service::call::<_, Root, _>(args, writer).await?,
        _ => create_json_file_service::call::<_, serde_json::Value, _>(args, writer).await?,
    };

    Ok(result)
}
