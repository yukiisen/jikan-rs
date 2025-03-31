use serde::Deserialize;

pub mod anime_genres;
pub mod manga_genres;

#[derive(Debug, Deserialize)]
pub enum GenreType {
    Genres,
    ExplicitGenres,
    Themes,
    Demographics,
}

impl GenreType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GenreType::Genres => "genres",
            GenreType::ExplicitGenres => "explicit_genres",
            GenreType::Themes => "themes",
            GenreType::Demographics => "demographics",
        }
    }
}
