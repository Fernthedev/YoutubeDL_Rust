use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct YoutubeDlVideoJson {
    pub id: String,
    pub title: String,
    pub formats: Vec<YoutubeDlFormatJson>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct YoutubeDlFormatJson {
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub acodec: Option<String>,
    pub format_note: Option<String>,
    pub url: String,
    pub ext: String,
}
