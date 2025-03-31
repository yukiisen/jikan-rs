use reqwest::get;
use reqwest::Url;

use anyhow::Result;
use anyhow::Context;

use crate::API_URL;
use crate::models::characters::*;

pub async fn get_anime_characters (id: u32) -> Result<CharactersResponse> {
    let url = Url::parse(format!("{API_URL}anime/{id}/characters").as_str())?;
    let res = get(url).await?;
    res.json().await.context("Failed to deserialize data")
}

pub async fn get_character_full (id: u32) -> Result<CharacterFullResponse> {
    let url = Url::parse(format!("{API_URL}characters/{id}/full").as_str())?;
    let res = get(url).await?;
    res.json().await.context("Failed to deserialize data")
}

#[cfg(test)]
mod characters_tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_characters () {
        let chars = get_anime_characters(5081).await;

        println!("{chars:#?}");
    }

    #[tokio::test]
    async fn test_character_full () {
        let char = get_character_full(11).await;

        println!("{char:#?}");
    }
}
