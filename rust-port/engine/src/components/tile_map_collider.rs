use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::types::{Transform, Vec2};

/// Edge boundary for grounded check
const EDGE_BOUNDARY: f32 = 0.01;

/// Collision checker interface
pub trait CollisionChecker {
    fn is_solid(&self, x: i32, y: i32) -> bool;
}

/// TileMapCollider component for collision detection against tile maps
pub struct TileMapCollider {
    /// Entity transform (set externally)
    transform: Transform,
    /// Collision offset from entity position
    pub offset: Vec2,
}

impl Default for TileMapCollider {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            offset: Vec2::ZERO,
        }
    }
}

impl TileMapCollider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }

    /// Check if the entity is colliding with any solid tiles
    pub fn colliding(&self, checker: &impl CollisionChecker) -> bool {
        let pos = &self.transform.position;
        let scale = &self.transform.scale;

        // Generate test points based on entity scale
        let half_w = scale.x / 2.0;
        let half_h = scale.y / 2.0;

        let left = pos.x - half_w + self.offset.x;
        let right = pos.x + half_w + self.offset.x;
        let top = pos.y - half_h + self.offset.y;
        let bottom = pos.y + half_h + self.offset.y;

        // Check corners and edges
        let points = self.generate_test_points(left, right, top, bottom, scale);

        for (px, py) in points {
            let tile_x = px.floor() as i32;
            let tile_y = py.floor() as i32;
            if checker.is_solid(tile_x, tile_y) {
                return true;
            }
        }

        false
    }

    /// Check if the entity is on the ground (for jumping)
    pub fn grounded(&self, checker: &impl CollisionChecker) -> bool {
        let mut test_transform = self.transform.clone();
        test_transform.position.y += EDGE_BOUNDARY;

        let pos = &test_transform.position;
        let scale = &test_transform.scale;

        let half_w = scale.x / 2.0;
        let half_h = scale.y / 2.0;

        let left = pos.x - half_w + self.offset.x;
        let right = pos.x + half_w + self.offset.x;
        let bottom = pos.y + half_h + self.offset.y;

        // Check bottom edge
        let steps = (scale.x.ceil() as i32).max(1) + 1;
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let px = left + t * (right - left);
            let tile_x = px.floor() as i32;
            let tile_y = bottom.floor() as i32;
            if checker.is_solid(tile_x, tile_y) {
                return true;
            }
        }

        false
    }

    /// Check collision at a specific position
    pub fn colliding_at(&self, x: f32, y: f32, checker: &impl CollisionChecker) -> bool {
        let scale = &self.transform.scale;

        let half_w = scale.x / 2.0;
        let half_h = scale.y / 2.0;

        let left = x - half_w + self.offset.x;
        let right = x + half_w + self.offset.x;
        let top = y - half_h + self.offset.y;
        let bottom = y + half_h + self.offset.y;

        let points = self.generate_test_points(left, right, top, bottom, scale);

        for (px, py) in points {
            let tile_x = px.floor() as i32;
            let tile_y = py.floor() as i32;
            if checker.is_solid(tile_x, tile_y) {
                return true;
            }
        }

        false
    }

    fn generate_test_points(
        &self,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        scale: &Vec2,
    ) -> Vec<(f32, f32)> {
        let mut points = Vec::new();

        // Generate a grid of test points based on scale
        let x_steps = (scale.x.ceil() as i32).max(1) + 1;
        let y_steps = (scale.y.ceil() as i32).max(1) + 1;

        for yi in 0..=y_steps {
            let t_y = yi as f32 / y_steps as f32;
            let py = top + t_y * (bottom - top);

            for xi in 0..=x_steps {
                let t_x = xi as f32 / x_steps as f32;
                let px = left + t_x * (right - left);
                points.push((px, py));
            }
        }

        points
    }
}

impl Component for TileMapCollider {
    impl_component_any!(TileMapCollider);

    fn name(&self) -> &'static str {
        "TileMapCollider"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, _ctx: &UpdateContext) {}

    fn render(&self, _ctx: &mut RenderContext) {}
}
