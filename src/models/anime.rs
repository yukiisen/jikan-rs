use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnimeResponse {
    pub data: Anime,
}

#[derive(Debug, Deserialize)]
pub struct FullAnimeResponse {
    pub data: AnimeFull,
}

#[derive(Debug, Deserialize)]
pub struct AnimeSearchResponse {
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
pub struct Anime {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub trailer: Option<Trailer>,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub source: String,
    pub episodes: Option<u32>,
    pub status: String,
    pub airing: bool,
    pub aired: Aired,
    pub duration: String,
    pub rating: String,
    pub score: Option<f64>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: u32,
    pub members: u32,
    pub favorites: u32,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub broadcast: Broadcast,
    pub producers: Vec<Producer>,
    pub licensors: Vec<Producer>,
    pub studios: Vec<Producer>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<Genre>,
    pub themes: Vec<Genre>,
    pub demographics: Vec<Genre>,
}
#[derive(Debug, Deserialize)]
pub struct AnimeFull {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub trailer: Option<Trailer>,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    pub r#type: String,
    pub source: String,
    pub episodes: Option<u32>,
    pub status: String,
    pub airing: bool,
    pub aired: Aired,
    pub duration: String,
    pub rating: String,
    pub score: Option<f64>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: u32,
    pub members: u32,
    pub favorites: u32,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub broadcast: Broadcast,
    pub producers: Vec<Producer>,
    pub licensors: Vec<Producer>,
    pub studios: Vec<Producer>,
    pub genres: Vec<Genre>,
    pub explicit_genres: Vec<Genre>,
    pub themes: Vec<Genre>,
    pub demographics: Vec<Genre>,
    pub relations: Vec<Relation>,
    pub theme: Theme,
    pub external: Vec<External>,
    pub streaming: Vec<External>,
}

#[derive(Debug, Deserialize)]
pub struct Images {
    pub jpg: ImageSet,
    pub webp: ImageSet,
}

#[derive(Debug, Deserialize)]
pub struct ImageSet {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Trailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Title {
    pub r#type: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct Aired {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prop: AiredProp,
    pub string: String,
}

#[derive(Debug, Deserialize)]
pub struct AiredProp {
    pub from: Date,
    pub to: Date,
}

#[derive(Debug, Deserialize)]
pub struct Date {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Broadcast {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
    pub string: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Producer {
    pub mal_id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub mal_id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Relation {
    pub relation: String,
    pub entry: Vec<Producer>,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct External {
    pub name: String,
    pub url: String,
}

