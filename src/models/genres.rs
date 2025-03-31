use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GenreResponse {
    pub data: Vec<GenreEntry>,
}

#[derive(Debug, Deserialize)]
pub struct GenreEntry {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub count: u32,
}
