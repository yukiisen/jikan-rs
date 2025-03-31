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
pub struct Character {
    pub mal_id: u32,
    pub url: String,
    pub images: CharacterImages,
    pub name: String,
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

