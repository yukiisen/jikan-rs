use std::collections::HashMap;

use reqwest::get;
use reqwest::Url;
use anyhow::Result;
use anyhow::Context;

use crate::models::list::*;
use crate::API_URL;

pub async fn get_schedule (filter: Weekday, params: Option<&HashMap<&str, &str>>) -> Result<AnimeListResponse> {
    let mut url = Url::parse(format!("{API_URL}schedules").as_str())?;
    let mut pairs = url.query_pairs_mut();
    pairs.append_pair("filter", filter.as_str());

    for (key, value) in params.unwrap_or(&HashMap::new()) {
        pairs.append_pair(key, value);
    }

    drop(pairs);

    let res = get(url).await?;
    res.json().await.context("Failed to deserialize json")
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Unknown,
    Other,
}

impl Weekday {
    pub fn as_str(&self) -> &str {
        match self {
            Weekday::Monday => "monday",
            Weekday::Tuesday => "tuesday",
            Weekday::Wednesday => "wednesday",
            Weekday::Thursday => "thursday",
            Weekday::Friday => "friday",
            Weekday::Saturday => "saturday",
            Weekday::Sunday => "sunday",
            Weekday::Unknown => "unknown",
            Weekday::Other => "other",
        }
    }
}

#[cfg(test)]
mod schedule_test {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_schedules () {
        let schedule = get_schedule(Weekday::Wednesday, None).await.unwrap();

        println!("{schedule:#?}");
    }
}
