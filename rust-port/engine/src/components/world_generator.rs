use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::types::{encode_address, Address, Vec2};
use noise::{NoiseFn, Simplex};
use std::collections::HashSet;

const CHUNK_SIZE: i32 = 32;

/// World generator component using Simplex noise
pub struct WorldGenerator {
    simplex: Simplex,
    seed: u32,
    generated_chunks: HashSet<Address>,
    /// Pending tiles to set: (x, y, tile_id)
    pub pending_tiles: Vec<(i32, i32, u8)>,
    /// Visible bounds for generation (set externally)
    pub visible_bounds: Option<(Vec2, Vec2)>,
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self::new(42)
    }
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            simplex: Simplex::new(seed),
            seed,
            generated_chunks: HashSet::new(),
            pending_tiles: Vec::new(),
            visible_bounds: None,
        }
    }

    pub fn set_visible_bounds(&mut self, min: Vec2, max: Vec2) {
        self.visible_bounds = Some((min, max));
    }

    /// Take pending tiles
    pub fn take_pending_tiles(&mut self) -> Vec<(i32, i32, u8)> {
        std::mem::take(&mut self.pending_tiles)
    }

    fn normalized_noise(&self, x: f64, seed_offset: u32) -> f64 {
        let value = self.simplex.get([x, seed_offset as f64]);
        (value + 1.0) / 2.0 // Normalize from [-1, 1] to [0, 1]
    }

    fn generate_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        let chunk_addr = encode_address(chunk_x, chunk_y);
        if self.generated_chunks.contains(&chunk_addr) {
            return;
        }
        self.generated_chunks.insert(chunk_addr);

        let world_x = chunk_x * CHUNK_SIZE;
        let world_y = chunk_y * CHUNK_SIZE;

        // First pass: terrain generation
        let mut tiles: Vec<(i32, i32, u8)> = Vec::new();

        for ty in 0..CHUNK_SIZE {
            for tx in 0..CHUNK_SIZE {
                let x = world_x + tx;
                let y = world_y + ty;

                // Calculate terrain height using noise
                let x_norm = x as f64 / 50.0;
                let mut noise = self.normalized_noise(x_norm, self.seed);
                noise += self.normalized_noise(x_norm.abs().sqrt() * x_norm.signum(), self.seed) / 2.0;
                noise += self.normalized_noise(x_norm.abs().powf(0.25) * x_norm.signum(), self.seed) / 4.0;

                let landstrip_height = (noise * 10.0).floor() as i32;
                let stone_height = landstrip_height + 5;

                // Determine tile type based on depth
                let tile_id = if y > stone_height {
                    // Below stone level - check for caves
                    let cave_noise = self.cave_noise(x, y);
                    if cave_noise < 0.42 {
                        0 // Cave (empty)
                    } else {
                        3 // Stone
                    }
                } else if y > landstrip_height {
                    // Dirt layer
                    1 // Dirt
                } else if y == landstrip_height {
                    // Surface
                    2 // Grass
                } else {
                    // Above ground
                    0 // Air
                };

                if tile_id > 0 {
                    tiles.push((x, y, tile_id));
                }
            }
        }

        // Second pass: cellular automata smoothing for caves
        // (simplified - full implementation would iterate multiple times)
        let smoothed = self.smooth_caves(&tiles, world_x, world_y);

        self.pending_tiles.extend(smoothed);
    }

    fn cave_noise(&self, x: i32, y: i32) -> f64 {
        let scale1 = 40.0;
        let scale2 = 20.0;
        let scale3 = 10.0;

        let n1 = self.simplex.get([x as f64 / scale1, y as f64 / scale1]);
        let n2 = self.simplex.get([x as f64 / scale2, y as f64 / scale2]);
        let n3 = self.simplex.get([x as f64 / scale3, y as f64 / scale3]);

        // Combine noise at different frequencies
        let combined = (n1 + 1.0) / 2.0 * 0.5 + (n2 + 1.0) / 2.0 * 0.3 + (n3 + 1.0) / 2.0 * 0.2;

        combined
    }

    fn smooth_caves(
        &self,
        tiles: &[(i32, i32, u8)],
        _chunk_x: i32,
        _chunk_y: i32,
    ) -> Vec<(i32, i32, u8)> {
        // Create a set for quick lookup
        let tile_set: HashSet<(i32, i32)> = tiles.iter().map(|(x, y, _)| (*x, *y)).collect();

        let mut result = Vec::new();

        for (x, y, tile_id) in tiles {
            // Count solid neighbors
            let mut solid_count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if tile_set.contains(&(x + dx, y + dy)) {
                        solid_count += 1;
                    }
                }
            }

            // Cellular automata rule: 5+ neighbors = solid
            if solid_count >= 5 || *tile_id == 2 {
                // Keep grass tiles always
                result.push((*x, *y, *tile_id));
            }
        }

        result
    }

    fn generate_visible_chunks(&mut self) {
        if let Some((min, max)) = self.visible_bounds {
            let chunk_min_x = (min.x as i32).div_euclid(CHUNK_SIZE) - 1;
            let chunk_max_x = (max.x as i32).div_euclid(CHUNK_SIZE) + 1;
            let chunk_min_y = (min.y as i32).div_euclid(CHUNK_SIZE) - 1;
            let chunk_max_y = (max.y as i32).div_euclid(CHUNK_SIZE) + 1;

            for chunk_y in chunk_min_y..=chunk_max_y {
                for chunk_x in chunk_min_x..=chunk_max_x {
                    self.generate_chunk(chunk_x, chunk_y);
                }
            }
        }
    }
}

impl Component for WorldGenerator {
    impl_component_any!(WorldGenerator);

    fn name(&self) -> &'static str {
        "WorldGenerator"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, _ctx: &UpdateContext) {
        // Generate chunks that are now visible
        self.generate_visible_chunks();
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // WorldGenerator doesn't render
    }
}
