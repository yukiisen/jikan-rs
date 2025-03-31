use reqwest::get;
use reqwest::Url;

use anyhow::Result;
use anyhow::Context;

use crate::models::episodes::*;
use crate::API_URL;

pub async fn get_anime_episodes (id: u32, page: Option<u32>) -> Result<EpisodesResponse> {
    let mut url = Url::parse(format!("{API_URL}anime/{id}/episodes").as_str())?;
    if page.is_some() { url.query_pairs_mut().append_pair("page", page.unwrap().to_string().as_str()); };

    let url = url.to_string();
    let res = get(url).await?;

    res.json().await.context("Failed to decode response")
}

pub async fn get_episode_by_id (anime_id: u32, ep_id: u32) -> Result<EpisodeResponse> {
    let url = Url::parse(format!("{API_URL}anime/{anime_id}/episodes/{ep_id}").as_str())?;

    let res = get(url).await?;

    res.json().await.context("Failed to decode response")
}

#[cfg(test)]
mod episode_test {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_episodes () {
        let eps = get_anime_episodes(5081, None).await;

        println!("{eps:#?}");
    }

    #[tokio::test]
    async fn test_episode_by_id () {
        let ep = get_episode_by_id(5081, 5).await;

        println!("{ep:#?}");
    }
}
