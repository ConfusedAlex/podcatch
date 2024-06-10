use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Podcast {
    pub name: String,
    pub episodes: Vec<Episode>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    // pub duration: i32,
    pub title: String,
    pub guid: String,
    pub description: String,
    pub mp3: String,
}
