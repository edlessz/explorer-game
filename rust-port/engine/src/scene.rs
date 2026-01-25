use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Scene definition loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub entities: Vec<SceneEntity>,
}

/// Entity definition within a scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneEntity {
    pub tag: Option<String>,
    #[serde(default)]
    pub components: HashMap<String, serde_json::Value>,
}

/// Tile registry entry definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileRegistryEntry {
    #[serde(rename = "tileId")]
    pub tile_id: u8,
    pub name: String,
    #[serde(rename = "assetPath")]
    pub asset_path: String,
    #[serde(default)]
    pub solid: bool,
    #[serde(rename = "lightIntensity")]
    pub light_intensity: Option<f32>,
    #[serde(rename = "lightRadius")]
    pub light_radius: Option<f32>,
}

impl Scene {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl TileRegistryEntry {
    pub fn from_json_array(json: &str) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_str(json)
    }
}
