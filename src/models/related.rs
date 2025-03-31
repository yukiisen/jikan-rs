use serde::Deserialize;

use super::anime::Images;

#[derive(Debug, Deserialize)]
pub struct RelationsResponse {
    pub data: Vec<Relation>,
}

#[derive(Debug, Deserialize)]
pub struct RecommendationsResponse {
    pub data: Vec<Recommendation>,
}

#[derive(Debug, Deserialize)]
pub struct Recommendation {
    pub entry: RecommendationEntry,
}

#[derive(Debug, Deserialize)]
pub struct RecommendationEntry {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct Relation {
    pub relation: String,
    pub entry: Vec<RelationEntry>,
}

#[derive(Debug, Deserialize)]
pub struct RelationEntry {
    pub mal_id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String,
}

