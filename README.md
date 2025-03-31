# Jikan-rs

Jikan.rs is a lightweight, no-cache Rust wrapper for the [Jikan API](https://jikan.moe/). It provides simple methods to interact with MyAnimeList's public data.

## Features
- Pure API wrapper, no caching.
- Simple and minimal.
- Covers various Jikan endpoints.

## Installation
Add Jikan.rs to your `Cargo.toml`:

```toml
[dependencies]
jikan-rs = { git: "https://github.com/yukiisen/jikan-rs" }
```

## Public API:

### `src/anime/characters.rs`
- `get_anime_characters (id: u32) -> Result<CharactersResponse>`
- `get_character_full (id: u32) -> Result<CharacterFullResponse>`

### `src/anime/episodes.rs`
- `get_anime_episodes (id: u32, page: Option<u32>) -> Result<EpisodesResponse>`
- `get_episode_by_id (anime_id: u32, ep_id: u32) -> Result<EpisodeResponse>`

### `src/anime/metadata.rs`
- `get_anime_full (id: u32) -> Result<FullAnimeResponse, reqwest::Error>`
- `get_anime (id: u32) -> Result<AnimeResponse, reqwest::Error>`
- `search_anime (query: &str, limit: Option<u32>, params: Option<&HashMap<&str, &str>>) -> Result<AnimeSearchResponse>`
- `get_anime_moreinfo (id: u32) -> Result<MoreInfoResponse>`
- `get_anime_stats (id: u32) -> Result<AnimeStatisticsResponse>`

### `src/anime/related.rs`
- `get_anime_relations (id: u32) -> Result<RelationsResponse>`
- `get_anime_recommendations (id: u32) -> Result<RecommendationsResponse>`
- `get_recommendations (page: u32) -> Result<RecommendedFullEntry>`
- `get_recommendations_manga (page: u32) -> Result<RecommendedFullEntry>`

### `src/genres/anime_genres.rs`
- `get_genre (filter: Option<GenreType>) -> Result<GenreResponse>`

### `src/genres/manga_genres.rs`
- `get_genre (filter: Option<GenreType>) -> Result<GenreResponse>`

### `src/schedules/mod.rs`
- `get_schedule (filter: Weekday, params: Option<&HashMap<&str, &str>>) -> Result<AnimeListResponse>`

### `src/seasons/mod.rs`
- `get_current_season (continuing: bool, page: u16, limit: u32, filter: AnimeType) -> Result<AnimeListResponse>`
- `get_season (year: u16, season: Season, continuing: bool, page: u16, filter: AnimeType) -> Result<AnimeListResponse>`
- `get_seasons () -> Result<SeasonListResponse>`
- `get_upcoming_season (page: u16, limit: u32, filter: AnimeType) -> Result<AnimeListResponse>`

### `src/top/mod.rs`
- `get_top_animes (params: &HashMap<&str, &str>) -> Result<AnimeListResponse>`
- `get_top_manga (params: &HashMap<&str, &str>) -> Result<AnimeListResponse>`

## License
MIT
