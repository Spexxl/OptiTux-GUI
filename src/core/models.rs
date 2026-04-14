use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GamePlatform {
    Steam,
    Heroic,
    Lutris,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub install_path: String,
    pub executable_path: Option<String>,
    pub upscalars: Vec<String>,
    pub platform: GamePlatform,
    pub app_id: String,
}
