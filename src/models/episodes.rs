use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EpisodesResponse {
    pub data: Vec<Episode>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct EpisodeResponse {
    pub data: EpisodeFull,
}

#[derive(Debug, Deserialize)]
pub struct EpisodeFull {
    pub mal_id: u32,
    pub url: String,
    pub title: String,
    pub title_japanese: String,
    pub title_romanji: String,
    pub duration: u32,
    pub aired: String,
    pub filler: bool,
    pub recap: bool,
    pub synopsis: String,
}

#[derive(Debug, Deserialize)]
pub struct Episode {
    pub mal_id: u32,
    pub url: Option<String>,
    pub title: String,
    pub title_japanese: String,
    pub title_romanji: String,
    pub aired: String,
    pub score: Option<f64>,
    pub filler: bool,
    pub recap: bool,
    pub forum_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub last_visible_page: u32,
    pub has_next_page: bool,
}
