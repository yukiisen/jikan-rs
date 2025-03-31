use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RelationsResponse {
    pub data: Vec<Relation>,
}

#[derive(Debug, Deserialize)]
pub struct Relation {
    pub relation: String,
    pub entry: Vec<RelationEntry>,
}

#[derive(Debug, Deserialize)]
pub struct RelationEntry {
    pub mal_id: u32,
    pub type_: String,
    pub name: String,
    pub url: String,
}

