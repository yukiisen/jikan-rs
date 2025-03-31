use reqwest::get;
use reqwest::Url;

use anyhow::Result;
use anyhow::Context;

use crate::models::related::*;
use crate::API_URL;

pub async fn get_anime_relations (id: u32) -> Result<RelationsResponse> {
    let url = Url::parse(format!("{API_URL}anime/{id}/relations").as_str())?;

    let res = get(url).await?;

    res.json().await.context("Failed to parse json")
}

pub async fn get_anime_recommendations (id: u32) -> Result<RecommendationsResponse> {
    let url = Url::parse(format!("{API_URL}anime/{id}/recommendations").as_str())?;
    let res = get(url).await?;
    res.json().await.context("Failed to parse json")
}

#[cfg(test)]
mod related_test {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_relations () {
        let relations = get_anime_relations(5081).await;

        println!("{relations:#?}");
    }

    #[tokio::test]
    async fn test_recommended () {
        let recommeded = get_anime_recommendations(5081).await;

        println!("{recommeded:#?}");
    }
}
