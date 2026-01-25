use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::renderer::rgba;
use crate::types::{Transform, Vec2};

/// Simple colored rectangle renderer
pub struct ColorRenderer {
    pub color: u32,
    /// Entity transform reference (set externally before render)
    transform: Transform,
}

impl ColorRenderer {
    pub fn new(color: u32) -> Self {
        Self {
            color,
            transform: Transform::default(),
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(rgba(r, g, b, a))
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }
}

impl Default for ColorRenderer {
    fn default() -> Self {
        Self::new(0xFFFF0000) // Red
    }
}

impl Component for ColorRenderer {
    impl_component_any!(ColorRenderer);

    fn name(&self) -> &'static str {
        "ColorRenderer"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, _ctx: &UpdateContext) {}

    fn render(&self, ctx: &mut RenderContext) {
        let pos = &self.transform.position;
        let scale = &self.transform.scale;
        let ppu = ctx.ppu;

        // Calculate screen position and size
        // The entity position is the center of the object
        let half_w = scale.x / 2.0;
        let half_h = scale.y / 2.0;

        ctx.renderer.push_transform();
        ctx.renderer.fill_rect(
            pos.x - half_w,
            pos.y - half_h,
            scale.x,
            scale.y,
            self.color,
        );
        ctx.renderer.pop_transform();
    }
}
