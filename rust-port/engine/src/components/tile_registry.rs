use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::scene::TileRegistryEntry;
use std::collections::HashMap;

/// Stores tile metadata for a tilemap
pub struct TileRegistry {
    entries: HashMap<u8, TileRegistryEntry>,
}

impl Default for TileRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TileRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register_tiles(&mut self, entries: Vec<TileRegistryEntry>) {
        for entry in entries {
            self.entries.insert(entry.tile_id, entry);
        }
    }

    pub fn get_entry(&self, tile_id: u8) -> Option<&TileRegistryEntry> {
        self.entries.get(&tile_id)
    }

    pub fn is_solid(&self, tile_id: u8) -> bool {
        self.entries
            .get(&tile_id)
            .map(|e| e.solid)
            .unwrap_or(tile_id > 0) // Default: any non-zero tile is solid
    }

    pub fn get_light_properties(&self, tile_id: u8) -> Option<(f32, f32)> {
        self.entries.get(&tile_id).and_then(|e| {
            match (e.light_intensity, e.light_radius) {
                (Some(intensity), Some(radius)) => Some((intensity, radius)),
                _ => None,
            }
        })
    }

    pub fn get_asset_path(&self, tile_id: u8) -> Option<&str> {
        self.entries.get(&tile_id).map(|e| e.asset_path.as_str())
    }
}

impl Component for TileRegistry {
    impl_component_any!(TileRegistry);

    fn name(&self) -> &'static str {
        "TileRegistry"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, _ctx: &UpdateContext) {}

    fn render(&self, _ctx: &mut RenderContext) {}
}
