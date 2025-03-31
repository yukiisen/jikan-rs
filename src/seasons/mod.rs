use reqwest::get;
use reqwest::Url;
use anyhow::Result;
use anyhow::Context;

use crate::models::list::*;
use crate::API_URL;

pub enum AnimeType {
    Tv,
    Movie,
    Ova,
    Special,
    Ona,
    Music,
    All
}

impl AnimeType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tv => "tv",
            Self::Movie => "movie",
            Self::Ova => "ova",
            Self::Special => "special",
            Self::Ona => "ona",
            Self::Music => "music",
            Self::All => "all"
        }
    }
}

pub enum Season {
    Winter,
    Summer,
    Fall,
    Spring,
}

impl Season {
    pub fn as_str (&self) -> &str {
        match self {
            Self::Winter => "winter",
            Self::Summer => "summer",
            Self::Spring => "spring",
            Self::Fall => "fall"
        }
    }
}

pub async fn get_current_season (continuing: bool, page: u16, limit: u32, filter: AnimeType) -> Result<AnimeListResponse> {
    let mut url = Url::parse(format!("{API_URL}seasons/now").as_str())?;
    let mut pairs = url.query_pairs_mut();
    
    if continuing { pairs.append_key_only("continuing"); };
    pairs.append_pair("page", page.to_string().as_str());
    pairs.append_pair("limit", limit.to_string().as_str());
    pairs.append_pair("type", filter.as_str());

    drop(pairs);

    let res = get(url).await?;
    res.json().await.context("Failed to deserialize data")
}

pub async fn get_season (year: u16, season: Season, continuing: bool, page: u16, filter: AnimeType) -> Result<AnimeListResponse> {
    let mut url = Url::parse(format!("{API_URL}seasons/{year}/{}", season.as_str()).as_str())?;
    let mut pairs = url.query_pairs_mut();

    pairs.append_pair("page", format!("{}", page).as_str());
    if !matches!(filter, AnimeType::All) { pairs.append_pair("filter", filter.as_str()); };
    if continuing { pairs.append_key_only("continuing"); };

    drop(pairs);

    let res = get(url).await?;

    res.json().await.context("Failed to deserialize data")
}

#[cfg(test)]
mod seasons_test {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_current_season () {
        let animes = get_current_season(true, 1, 10, AnimeType::All).await.unwrap();

        println!("{animes:#?}");
    }

    #[tokio::test]
    async fn test_get_season () {
        let result = get_season(2022, Season::Fall, true, 1, AnimeType::All).await.unwrap();

        println!("{result:#?}");
    }
}
