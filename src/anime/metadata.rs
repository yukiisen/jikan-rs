use std::collections::HashMap;
use anyhow::Context;
use anyhow::Result;

use reqwest::get;
use reqwest::Url;

use crate::models::anime::*;
use crate::API_URL;

pub async fn get_anime_full (id: u32) -> Result<FullAnimeResponse, reqwest::Error> {
    let path = format!("{API_URL}anime/{id}/full");
    let res = get(path).await?;

    res.json().await
}

pub async fn get_anime (id: u32) -> Result<AnimeResponse, reqwest::Error> {
    let path = format!("{API_URL}anime/{id}");
    let res = get(path).await?;

    res.json().await
}

pub async fn search_anime (query: &str, limit: Option<u32>, params: Option<&HashMap<&str, &str>>) -> Result<AnimeSearchResponse> {
    let mut url = Url::parse(format!("{API_URL}anime").as_str())?;
    let mut queries = url.query_pairs_mut();

    queries.append_pair("q", query);
    if let Some(limit) = limit { queries.append_pair("limit", limit.to_string().as_str()); };
    
    for (key, val) in params.unwrap_or(&HashMap::new()) {
        queries.append_pair(key, val);
    }

    drop(queries);
    let url = url.to_string();

    let res = get(url).await?;

    res.json().await.map_err(anyhow::Error::from)
}

pub async fn get_anime_moreinfo (id: u32) -> Result<MoreInfoResponse> {
    let url = Url::parse(format!("{API_URL}anime/{id}/moreinfo").as_str())?;

    let res = get(url).await?;

    res.json().await.context("Failed To deserialize response")
}

pub async fn get_anime_stats (id: u32) -> Result<AnimeStatisticsResponse> {
    let url = Url::parse(format!("{API_URL}anime/{id}/statistics").as_str())?;

    let res = get(url).await?;

    res.json().await.context("Failed to deserialize data")
}

#[cfg(test)]
mod anime_tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_anime_full () {
        let anime = get_anime_full(5081).await;

        println!("{anime:#?}");

        assert!(anime.is_ok() == true);
    }

    #[tokio::test]
    async fn test_anime () {
        let anime = get_anime(5081).await;

        println!("{anime:#?}");
        assert!(anime.is_ok() == true);
    }

    #[tokio::test]
    async fn test_anime_search () {
        let results = search_anime("kaguya", Some(5), None).await;

        if let Ok(res) = results {
            println!("{:#?}", res.data.iter().map(|e| &e.title).collect::<Vec<&String>>());
        } else {
            println!("{results:#?}");
        }
    }

    #[tokio::test]
    async fn test_anime_moreinfo () {
        let info = get_anime_moreinfo(5081).await;

        if let Ok(res) = info {
            println!("{}", res.data.moreinfo);
        } else {
            println!("{info:#?}");
        }
    }

    #[tokio::test]
    async fn test_anime_stats () {
        let stats = get_anime_stats(5081).await;

        println!("{stats:#?}");
    }
}
