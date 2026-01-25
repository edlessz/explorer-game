use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::renderer::rgb;
use crate::types::{encode_address, Address, Vec2};
use std::collections::HashMap;

const CHUNK_SIZE: i32 = 32;
const CACHE_PPU: i32 = 32; // Pixels per unit in chunk cache

/// Cached chunk data
struct ChunkCache {
    buffer: Vec<u32>,
    is_dirty: bool,
}

impl ChunkCache {
    fn new() -> Self {
        let size = (CHUNK_SIZE * CACHE_PPU) as usize;
        Self {
            buffer: vec![0; size * size],
            is_dirty: true,
        }
    }
}

/// Tile map component for storing and rendering tiles
pub struct TileMap {
    tiles: HashMap<Address, u8>,
    chunk_cache: HashMap<Address, ChunkCache>,
    dirty_chunks: Vec<Address>,
    /// Tile colors for rendering (simplified - would use textures in full impl)
    tile_colors: HashMap<u8, u32>,
    /// Reference to light map data (tile address -> light level 0.0-1.0)
    light_data: HashMap<Address, f32>,
}

impl Default for TileMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TileMap {
    pub fn new() -> Self {
        let mut tile_colors = HashMap::new();
        // Default tile colors
        tile_colors.insert(1, rgb(139, 90, 43)); // Dirt - brown
        tile_colors.insert(2, rgb(34, 139, 34));  // Grass - green
        tile_colors.insert(3, rgb(128, 128, 128)); // Stone - gray
        tile_colors.insert(4, rgb(255, 255, 200)); // Light - pale yellow

        Self {
            tiles: HashMap::new(),
            chunk_cache: HashMap::new(),
            dirty_chunks: Vec::new(),
            tile_colors,
            light_data: HashMap::new(),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> u8 {
        let addr = encode_address(x, y);
        *self.tiles.get(&addr).unwrap_or(&0)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile_id: u8) {
        let addr = encode_address(x, y);

        if tile_id == 0 {
            self.tiles.remove(&addr);
        } else {
            self.tiles.insert(addr, tile_id);
        }

        // Mark chunk as dirty
        let chunk_x = x.div_euclid(CHUNK_SIZE);
        let chunk_y = y.div_euclid(CHUNK_SIZE);
        let chunk_addr = encode_address(chunk_x, chunk_y);

        if let Some(cache) = self.chunk_cache.get_mut(&chunk_addr) {
            cache.is_dirty = true;
        }

        if !self.dirty_chunks.contains(&chunk_addr) {
            self.dirty_chunks.push(chunk_addr);
        }
    }

    pub fn set_light_data(&mut self, data: HashMap<Address, f32>) {
        self.light_data = data;
        // Mark all chunks as dirty when lighting changes
        for cache in self.chunk_cache.values_mut() {
            cache.is_dirty = true;
        }
    }

    pub fn get_light(&self, x: i32, y: i32) -> f32 {
        let addr = encode_address(x, y);
        *self.light_data.get(&addr).unwrap_or(&0.3) // Default ambient light
    }

    pub fn tiles(&self) -> &HashMap<Address, u8> {
        &self.tiles
    }

    fn get_chunk_addr(x: i32, y: i32) -> Address {
        let chunk_x = x.div_euclid(CHUNK_SIZE);
        let chunk_y = y.div_euclid(CHUNK_SIZE);
        encode_address(chunk_x, chunk_y)
    }

    fn render_chunk_to_cache(&mut self, chunk_x: i32, chunk_y: i32) {
        let chunk_addr = encode_address(chunk_x, chunk_y);

        // Ensure cache exists
        if !self.chunk_cache.contains_key(&chunk_addr) {
            self.chunk_cache.insert(chunk_addr, ChunkCache::new());
        }

        // Check if dirty first (immutable borrow)
        let is_dirty = self.chunk_cache.get(&chunk_addr).map(|c| c.is_dirty).unwrap_or(false);
        if !is_dirty {
            return;
        }

        let cache_size = (CHUNK_SIZE * CACHE_PPU) as usize;
        let world_x = chunk_x * CHUNK_SIZE;
        let world_y = chunk_y * CHUNK_SIZE;

        // Pre-collect tile data (immutable borrows of self)
        let mut tile_data: Vec<(i32, i32, u8, u32, f32)> = Vec::new();
        for ty in 0..CHUNK_SIZE {
            for tx in 0..CHUNK_SIZE {
                let tile_x = world_x + tx;
                let tile_y = world_y + ty;
                let tile_id = self.get_tile(tile_x, tile_y);

                if tile_id > 0 {
                    let base_color = *self.tile_colors.get(&tile_id).unwrap_or(&rgb(255, 0, 255));
                    let light = self.get_light(tile_x, tile_y);
                    tile_data.push((tx, ty, tile_id, base_color, light));
                }
            }
        }

        // Now get mutable borrow and render
        let cache = self.chunk_cache.get_mut(&chunk_addr).unwrap();

        // Clear cache to transparent
        cache.buffer.fill(0);

        // Render collected tile data
        for (tx, ty, _tile_id, base_color, light) in tile_data {
            // Apply lighting
            let r = (((base_color >> 16) & 0xFF) as f32 * light) as u8;
            let g = (((base_color >> 8) & 0xFF) as f32 * light) as u8;
            let b = ((base_color & 0xFF) as f32 * light) as u8;
            let color = rgb(r, g, b);

            // Fill tile in cache (CACHE_PPU pixels per tile)
            let px_start_x = (tx * CACHE_PPU) as usize;
            let px_start_y = (ty * CACHE_PPU) as usize;

            for py in 0..CACHE_PPU as usize {
                for px in 0..CACHE_PPU as usize {
                    let idx = (px_start_y + py) * cache_size + (px_start_x + px);
                    cache.buffer[idx] = color;
                }
            }
        }

        cache.is_dirty = false;
    }

    pub fn render_visible_chunks(
        &mut self,
        renderer: &mut crate::renderer::Renderer,
        camera_bounds: (Vec2, Vec2),
        ppu: Vec2,
        camera_pos: Vec2,
    ) {
        let (min, max) = camera_bounds;

        // Calculate visible chunk range
        let chunk_min_x = (min.x as i32).div_euclid(CHUNK_SIZE) - 1;
        let chunk_max_x = (max.x as i32).div_euclid(CHUNK_SIZE) + 1;
        let chunk_min_y = (min.y as i32).div_euclid(CHUNK_SIZE) - 1;
        let chunk_max_y = (max.y as i32).div_euclid(CHUNK_SIZE) + 1;

        let screen_width = renderer.width() as f32;
        let screen_height = renderer.height() as f32;

        for chunk_y in chunk_min_y..=chunk_max_y {
            for chunk_x in chunk_min_x..=chunk_max_x {
                // Render chunk to cache if dirty
                self.render_chunk_to_cache(chunk_x, chunk_y);

                let chunk_addr = encode_address(chunk_x, chunk_y);
                if let Some(cache) = self.chunk_cache.get(&chunk_addr) {
                    // Calculate screen position for this chunk
                    let world_x = (chunk_x * CHUNK_SIZE) as f32;
                    let world_y = (chunk_y * CHUNK_SIZE) as f32;

                    // World to screen conversion
                    let screen_x = ((world_x - camera_pos.x) * ppu.x + screen_width / 2.0) as i32;
                    let screen_y = ((world_y - camera_pos.y) * ppu.y + screen_height / 2.0) as i32;

                    // Calculate scale factor between cache PPU and render PPU
                    let scale_x = ppu.x / CACHE_PPU as f32;
                    let scale_y = ppu.y / CACHE_PPU as f32;

                    // Blit the chunk with scaling
                    self.blit_scaled(
                        renderer,
                        &cache.buffer,
                        (CHUNK_SIZE * CACHE_PPU) as usize,
                        (CHUNK_SIZE * CACHE_PPU) as usize,
                        screen_x,
                        screen_y,
                        scale_x,
                        scale_y,
                    );
                }
            }
        }
    }

    fn blit_scaled(
        &self,
        renderer: &mut crate::renderer::Renderer,
        src: &[u32],
        src_width: usize,
        src_height: usize,
        dst_x: i32,
        dst_y: i32,
        scale_x: f32,
        scale_y: f32,
    ) {
        let dst_width = (src_width as f32 * scale_x) as i32;
        let dst_height = (src_height as f32 * scale_y) as i32;

        for dy in 0..dst_height {
            let screen_y = dst_y + dy;
            if screen_y < 0 || screen_y >= renderer.height() as i32 {
                continue;
            }

            let src_y = ((dy as f32) / scale_y) as usize;
            if src_y >= src_height {
                continue;
            }

            for dx in 0..dst_width {
                let screen_x = dst_x + dx;
                if screen_x < 0 || screen_x >= renderer.width() as i32 {
                    continue;
                }

                let src_x = ((dx as f32) / scale_x) as usize;
                if src_x >= src_width {
                    continue;
                }

                let src_idx = src_y * src_width + src_x;
                let color = src[src_idx];

                // Only draw non-transparent pixels
                if color != 0 {
                    renderer.set_pixel(screen_x, screen_y, color);
                }
            }
        }
    }
}

impl Component for TileMap {
    impl_component_any!(TileMap);

    fn name(&self) -> &'static str {
        "TileMap"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, _ctx: &UpdateContext) {}

    fn render(&self, _ctx: &mut RenderContext) {
        // Rendering is done via render_visible_chunks called by the game loop
    }
}
