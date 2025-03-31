use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CharactersResponse {
    pub data: Vec<CharacterEntry>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterEntry {
    pub character: Character,
    pub role: String,
    pub voice_actors: Vec<VoiceActor>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterFullResponse {
    pub data: Character
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub mal_id: u32,
    pub url: String,
    pub images: CharacterImages,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Vec<String>,
    pub favorites: u32,
    pub about: Option<String>,
    pub anime: Vec<CharacterAnime>,
    pub manga: Vec<CharacterManga>,
    pub voices: Vec<CharacterVoice>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterImages {
    pub jpg: CharacterImageDetails,
    pub webp: CharacterImageDetails,
}

#[derive(Debug, Deserialize)]
pub struct CharacterImageDetails {
    pub image_url: String,
    pub small_image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VoiceActor {
    pub person: VoiceActorPerson,
    pub language: String,
}

#[derive(Debug, Deserialize)]
pub struct VoiceActorPerson {
    pub mal_id: u32,
    pub url: String,
    pub images: VoiceActorImages,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct VoiceActorImages {
    pub jpg: VoiceActorImageDetails,
}

#[derive(Debug, Deserialize)]
pub struct VoiceActorImageDetails {
    pub image_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CharacterAnime {
    pub role: String,
    pub anime: MangaEntry,
}

#[derive(Debug, Deserialize)]
pub struct CharacterManga {
    pub role: String,
    pub manga: MangaEntry,
}

#[derive(Debug, Deserialize)]
pub struct CharacterVoice {
    pub language: String,
    pub person: VoiceActorPerson,
}

#[derive(Debug, Deserialize)]
pub struct MangaEntry {
    pub mal_id: u32,
    pub url: String,
    pub images: MangaImages,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaImages {
    pub jpg: MangaImageDetails,
    pub webp: MangaImageDetails,
}

#[derive(Debug, Deserialize)]
pub struct MangaImageDetails {
    pub image_url: String,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

