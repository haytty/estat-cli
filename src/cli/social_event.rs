use clap::{Parser};
use url::{Url};
use anyhow::Result;
use crate::lib::http::request::Requester;
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
}

const SOCIAL_EVENT_URL: &str = "https://dashboard.e-stat.go.jp/api/1.0/Json/getSocialEventInfo";

impl Requester for SocialEventArgs {
    fn to_url(&self) -> Result<Url> {
        let mut url = Url::parse(SOCIAL_EVENT_URL)?;

        let add_param = |url: &mut Url, key: &str, value: Option<&String>| {
            if let Some(val) = value {
                url.query_pairs_mut().append_pair(key, val);
            }
        };

        add_param(&mut url, "Lang", self.lang.as_ref());
        add_param(&mut url, "Time", self.time.as_ref());
        add_param(&mut url, "TimeFrom", self.time_from.as_ref());
        add_param(&mut url, "TimeTo", self.time_to.as_ref());
        add_param(&mut url, "SocialEventLevel", self.social_event_level.as_ref());
        add_param(&mut url, "Category", self.category.as_ref());
        add_param(&mut url, "ModifiedFrom", self.modified_from.as_ref());
        add_param(&mut url, "ModifiedTo", self.modified_to.as_ref());

        println!("{}", url);
        Ok(url)
    }
}

pub async fn handle(args: SocialEventArgs) -> Result<()> {
    let result = create_json_file_service::call::<_, _, Root>(args, "/tmp/social_event.json").await?;
    Ok(result)
}
