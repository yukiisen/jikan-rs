use super::anime::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnimeListResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Pagination { 
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub items: PaginationItems,
}

#[derive(Debug, Deserialize)]
pub struct PaginationItems { 
    pub count: u32,
    pub total: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct SeasonListResponse {
    pub data: Vec<SeasonEntry>
}

#[derive(Debug, Deserialize)]
pub struct SeasonEntry {
    pub year: u32,
    pub season: Vec<String>
}


#[derive(Debug, Deserialize)]
pub struct MangaListResponse {
    pub data: Vec<Manga>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Manga {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub r#type: String,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: String,
    pub publishing: bool,
    pub published: Published,
    pub score: Option<f64>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: u32,
    pub members: u32,
    pub favorites: u32,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub authors: Vec<Producer>,
    pub serializations: Vec<Producer>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<Genre>,
    pub themes: Vec<Genre>,
    pub demographics: Vec<Genre>,
}

#[derive(Debug, Deserialize)]
pub struct Published {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prop: PublishedProp,
    pub string: String,
}

#[derive(Debug, Deserialize)]
pub struct PublishedProp {
    pub from: Date,
    pub to: Date,
}

