use reqwest::get;
use reqwest::Url;

use anyhow::Result;
use anyhow::Context;

use crate::API_URL;
use crate::models::genres::*;
use super::GenreType;

pub async fn get_genre (filter: Option<GenreType>) -> Result<GenreResponse> {
    let mut url = Url::parse(format!("{API_URL}genres/anime").as_str())?;
    if let Some(filter) = filter { url.query_pairs_mut().append_pair("filter", &filter.as_str()); };
    let res = get(url).await?;
    res.json().await.context("Failed to Deserialize data")
}

