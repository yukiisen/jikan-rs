use std::collections::HashMap;

use reqwest::get;
use reqwest::Url;
use anyhow::Result;
use anyhow::Context;

use crate::API_URL;
use crate::models::list::*;

pub async fn get_top_animes (params: &HashMap<&str, &str>) -> Result<AnimeListResponse> {
    let mut url = Url::parse(format!("{API_URL}top/anime").as_str())?;
    let mut pairs = url.query_pairs_mut();

    for (key, val) in params {
        pairs.append_pair(key, val);
    }

    drop(pairs);

    let res = get(url).await?;
    res.json().await.context("Failed to deseialize data")
}

pub async fn get_top_manga (params: &HashMap<&str, &str>) -> Result<MangaListResponse> {
    let mut url = Url::parse(format!("{API_URL}top/manga").as_str())?;
    let mut pairs = url.query_pairs_mut();

    for (key, val) in params {
        pairs.append_pair(key, val);
    }

    drop(pairs);

    let res = get(url).await?;
    res.json().await.context("Failed to deseialize data")
}

#[cfg(test)]
mod top_test {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_top_animes () {
        let top = get_top_animes(&HashMap::new()).await.unwrap();

        println!("{top:#?}");
    }
}
