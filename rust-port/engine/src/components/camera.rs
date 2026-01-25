use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::types::{Vec2, Vec3};

/// Camera component for viewport management
pub struct Camera {
    /// Pixels per unit (zoom level) for X axis
    pub ppu_x: f32,
    /// Pixels per unit (zoom level) for Y axis
    pub ppu_y: f32,
    /// Viewport width in pixels (set during setup)
    viewport_width: usize,
    /// Viewport height in pixels (set during setup)
    viewport_height: usize,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            ppu_x: 16.0,
            ppu_y: 16.0,
            viewport_width: 800,
            viewport_height: 600,
        }
    }
}

impl Camera {
    pub fn new(ppu_x: f32, ppu_y: f32) -> Self {
        Self {
            ppu_x,
            ppu_y,
            ..Default::default()
        }
    }

    pub fn set_viewport_size(&mut self, width: usize, height: usize) {
        self.viewport_width = width;
        self.viewport_height = height;
    }

    /// Get the visible bounds in world coordinates given camera position
    pub fn get_bounds(&self, camera_pos: &Vec3, camera_scale: &Vec2) -> (Vec2, Vec2) {
        let half_width = (self.viewport_width as f32 / self.ppu_x / 2.0) / camera_scale.x;
        let half_height = (self.viewport_height as f32 / self.ppu_y / 2.0) / camera_scale.y;

        let min = Vec2::new(camera_pos.x - half_width, camera_pos.y - half_height);
        let max = Vec2::new(camera_pos.x + half_width, camera_pos.y + half_height);

        (min, max)
    }

    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(
        &self,
        screen_x: f32,
        screen_y: f32,
        camera_pos: &Vec3,
        camera_rotation: f32,
        camera_scale: &Vec2,
    ) -> Vec2 {
        // Convert to normalized device coordinates (-1 to 1)
        let ndc_x = (screen_x / self.viewport_width as f32) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen_y / self.viewport_height as f32) * 2.0;

        // Convert to world coordinates (before rotation)
        let half_width = self.viewport_width as f32 / self.ppu_x / 2.0 / camera_scale.x;
        let half_height = self.viewport_height as f32 / self.ppu_y / 2.0 / camera_scale.y;

        let world_x = camera_pos.x + ndc_x * half_width;
        let world_y = camera_pos.y - ndc_y * half_height;

        // Apply inverse rotation
        let cos = camera_rotation.cos();
        let sin = camera_rotation.sin();
        let dx = world_x - camera_pos.x;
        let dy = world_y - camera_pos.y;

        Vec2::new(
            camera_pos.x + dx * cos + dy * sin,
            camera_pos.y - dx * sin + dy * cos,
        )
    }

    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(
        &self,
        world_x: f32,
        world_y: f32,
        camera_pos: &Vec3,
        camera_rotation: f32,
        camera_scale: &Vec2,
    ) -> Vec2 {
        // Apply rotation
        let cos = camera_rotation.cos();
        let sin = camera_rotation.sin();
        let dx = world_x - camera_pos.x;
        let dy = world_y - camera_pos.y;
        let rotated_x = dx * cos - dy * sin;
        let rotated_y = dx * sin + dy * cos;

        // Convert to screen coordinates
        let screen_x = (rotated_x * self.ppu_x * camera_scale.x) + self.viewport_width as f32 / 2.0;
        let screen_y = (rotated_y * self.ppu_y * camera_scale.y) + self.viewport_height as f32 / 2.0;

        Vec2::new(screen_x, screen_y)
    }

    /// Round position to pixel boundary (for crisp rendering)
    pub fn round_to_pixel(&self, x: f32, y: f32) -> Vec2 {
        Vec2::new(
            (x * self.ppu_x).round() / self.ppu_x,
            (y * self.ppu_y).round() / self.ppu_y,
        )
    }

    pub fn ppu(&self) -> Vec2 {
        Vec2::new(self.ppu_x, self.ppu_y)
    }
}

impl Component for Camera {
    impl_component_any!(Camera);

    fn name(&self) -> &'static str {
        "Camera"
    }

    fn setup(&mut self, ctx: &SetupContext) {
        self.viewport_width = ctx.game_width;
        self.viewport_height = ctx.game_height;
    }

    fn update(&mut self, ctx: &UpdateContext) {
        self.viewport_width = ctx.game_width;
        self.viewport_height = ctx.game_height;
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // Camera doesn't render anything itself
    }
}
