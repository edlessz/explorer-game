use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::types::{decode_address, encode_address, Address};
use std::collections::{HashMap, HashSet};

const CHUNK_SIZE: i32 = 32;

/// Light source definition
#[derive(Clone, Debug)]
pub struct LightSource {
    pub x: i32,
    pub y: i32,
    pub intensity: f32,
    pub radius: f32,
}

/// LightMap component for dynamic lighting
pub struct LightMap {
    /// Calculated light values per tile
    lighting: HashMap<Address, f32>,
    /// Light sources
    light_sources: HashMap<Address, LightSource>,
    /// Which tiles are affected by which light sources
    light_source_dependencies: HashMap<Address, HashSet<Address>>,
    /// Which light sources affect each tile
    tile_light_sources: HashMap<Address, HashSet<Address>>,
    /// Dirty chunks that need recalculation
    dirty_chunks: HashSet<Address>,
    /// Ambient light level
    pub ambient_light: f32,
}

impl Default for LightMap {
    fn default() -> Self {
        Self {
            lighting: HashMap::new(),
            light_sources: HashMap::new(),
            light_source_dependencies: HashMap::new(),
            tile_light_sources: HashMap::new(),
            dirty_chunks: HashSet::new(),
            ambient_light: 0.1,
        }
    }
}

impl LightMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_light_source(&mut self, x: i32, y: i32, intensity: f32, radius: f32) {
        let addr = encode_address(x, y);
        self.light_sources.insert(
            addr,
            LightSource {
                x,
                y,
                intensity,
                radius,
            },
        );
        self.mark_light_source_dirty(x, y, radius);
    }

    pub fn remove_light_source(&mut self, x: i32, y: i32) {
        let addr = encode_address(x, y);
        if let Some(source) = self.light_sources.remove(&addr) {
            self.mark_light_source_dirty(x, y, source.radius);

            // Clear dependencies
            if let Some(affected) = self.light_source_dependencies.remove(&addr) {
                for tile_addr in affected {
                    if let Some(sources) = self.tile_light_sources.get_mut(&tile_addr) {
                        sources.remove(&addr);
                    }
                }
            }
        }
    }

    fn mark_light_source_dirty(&mut self, x: i32, y: i32, radius: f32) {
        let r = radius.ceil() as i32;

        // Mark all chunks in radius as dirty
        let chunk_min_x = (x - r).div_euclid(CHUNK_SIZE);
        let chunk_max_x = (x + r).div_euclid(CHUNK_SIZE);
        let chunk_min_y = (y - r).div_euclid(CHUNK_SIZE);
        let chunk_max_y = (y + r).div_euclid(CHUNK_SIZE);

        for cy in chunk_min_y..=chunk_max_y {
            for cx in chunk_min_x..=chunk_max_x {
                self.dirty_chunks.insert(encode_address(cx, cy));
            }
        }
    }

    pub fn mark_chunk_dirty(&mut self, chunk_x: i32, chunk_y: i32) {
        self.dirty_chunks.insert(encode_address(chunk_x, chunk_y));
    }

    /// Bake lighting for all dirty chunks
    pub fn bake(&mut self) {
        let dirty: Vec<Address> = self.dirty_chunks.drain().collect();

        for chunk_addr in dirty {
            self.bake_chunk(chunk_addr);
        }
    }

    fn bake_chunk(&mut self, chunk_addr: Address) {
        let (chunk_x, chunk_y) = decode_address(chunk_addr);
        let world_x = chunk_x * CHUNK_SIZE;
        let world_y = chunk_y * CHUNK_SIZE;

        // Find all light sources that could affect this chunk
        let mut affecting_sources: Vec<(Address, &LightSource)> = Vec::new();

        for (addr, source) in &self.light_sources {
            let radius = source.radius.ceil() as i32;

            // Check if source could affect any tile in this chunk
            if source.x + radius >= world_x
                && source.x - radius < world_x + CHUNK_SIZE
                && source.y + radius >= world_y
                && source.y - radius < world_y + CHUNK_SIZE
            {
                affecting_sources.push((*addr, source));
            }
        }

        // Calculate lighting for each tile in chunk
        for ty in 0..CHUNK_SIZE {
            for tx in 0..CHUNK_SIZE {
                let tile_x = world_x + tx;
                let tile_y = world_y + ty;
                let tile_addr = encode_address(tile_x, tile_y);

                let mut total_light = self.ambient_light;

                // Clear old source tracking
                self.tile_light_sources.remove(&tile_addr);

                for (source_addr, source) in &affecting_sources {
                    let dx = tile_x as f32 - source.x as f32;
                    let dy = tile_y as f32 - source.y as f32;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist <= source.radius {
                        let falloff = 1.0 - (dist / source.radius);
                        let contribution = source.intensity * falloff;
                        total_light += contribution;

                        // Track which sources affect this tile
                        self.tile_light_sources
                            .entry(tile_addr)
                            .or_insert_with(HashSet::new)
                            .insert(*source_addr);

                        // Track which tiles this source affects
                        self.light_source_dependencies
                            .entry(*source_addr)
                            .or_insert_with(HashSet::new)
                            .insert(tile_addr);
                    }
                }

                self.lighting.insert(tile_addr, total_light.min(1.0));
            }
        }
    }

    pub fn get_lighting(&self, x: i32, y: i32) -> f32 {
        let addr = encode_address(x, y);
        *self.lighting.get(&addr).unwrap_or(&self.ambient_light)
    }

    pub fn get_all_lighting(&self) -> HashMap<Address, f32> {
        self.lighting.clone()
    }

    /// Sync light sources from tile map (for tiles with light properties)
    pub fn sync_from_tiles<F>(&mut self, get_tile_light: F, bounds: ((i32, i32), (i32, i32)))
    where
        F: Fn(i32, i32) -> Option<(f32, f32)>,
    {
        let ((min_x, min_y), (max_x, max_y)) = bounds;

        // Clear existing tile-based light sources and re-scan
        let old_sources: Vec<Address> = self.light_sources.keys().cloned().collect();

        for addr in old_sources {
            let (x, y) = decode_address(addr);
            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                self.remove_light_source(x, y);
            }
        }

        // Add light sources from tiles
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some((intensity, radius)) = get_tile_light(x, y) {
                    self.add_light_source(x, y, intensity, radius);
                }
            }
        }
    }
}

impl Component for LightMap {
    impl_component_any!(LightMap);

    fn name(&self) -> &'static str {
        "LightMap"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, _ctx: &UpdateContext) {
        // Bake any dirty chunks
        self.bake();
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // LightMap doesn't render directly - it provides data to TileMap
    }
}
