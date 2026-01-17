use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
    pub name: String,
    pub description: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerCollection {
    pub stickers: Vec<Sticker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewStickerCollection {
    pub new_stickers: Vec<Sticker>,
}

impl Sticker {
    pub fn new(name: String, link: String, description: Option<String>) -> Self {
        Self {
            name,
            description: description.unwrap_or_else(|| "a cool sticker".to_string()),
            link,
        }
    }
}

impl StickerCollection {
    pub fn load_from_file(path: &str) -> Result<Self, Error> {
        if !Path::new(path).exists() {
            return Ok(StickerCollection {
                stickers: Vec::new(),
            });
        }

        let content = fs::read_to_string(path)?;
        let collection: StickerCollection = serde_json::from_str(&content)?;
        Ok(collection)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.stickers.iter().any(|s| s.name == name)
    }

    pub fn find(&self, name: &str) -> Option<&Sticker> {
        self.stickers.iter().find(|s| s.name == name)
    }
}

impl NewStickerCollection {
    pub fn new() -> Self {
        Self {
            new_stickers: Vec::new(),
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Error> {
        if !Path::new(path).exists() {
            return Ok(NewStickerCollection::new());
        }

        let content = fs::read_to_string(path)?;
        let collection: NewStickerCollection = serde_json::from_str(&content)?;
        Ok(collection)
    }
}
