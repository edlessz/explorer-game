use engine::component::{Component, InputContext, RenderContext, SetupContext, UpdateContext};
use engine::components::Camera;
use engine::impl_component_any;
use engine::input::KeyCode;
use engine::types::{Transform, Vec2, Vec3};

const DEFAULT_CAMERA_SPEED: f32 = 10.0;
const ZOOM_FACTOR: f32 = 1.1;

/// Camera controller component for following a target and zoom control
pub struct CameraController {
    pub camera_speed: f32,
    /// Target position to follow
    pub target_position: Vec3,
    /// Current camera position (smoothed)
    current_position: Vec3,
    /// Zoom change requested (-1 for out, 0 for none, 1 for in)
    zoom_change: i32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            camera_speed: DEFAULT_CAMERA_SPEED,
            target_position: Vec3::ZERO,
            current_position: Vec3::ZERO,
            zoom_change: 0,
        }
    }
}

impl CameraController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.camera_speed = speed;
        self
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target_position = target;
    }

    pub fn get_position(&self) -> Vec3 {
        self.current_position
    }

    /// Get zoom change and reset it
    pub fn take_zoom_change(&mut self) -> i32 {
        let change = self.zoom_change;
        self.zoom_change = 0;
        change
    }
}

impl Component for CameraController {
    impl_component_any!(CameraController);

    fn name(&self) -> &'static str {
        "CameraController"
    }

    fn setup(&mut self, _ctx: &SetupContext) {
        self.current_position = self.target_position;
    }

    fn update(&mut self, ctx: &UpdateContext) {
        // Smooth follow
        let dx = self.target_position.x - self.current_position.x;
        let dy = self.target_position.y - self.current_position.y;

        let follow_speed = self.camera_speed * ctx.delta_time;

        self.current_position.x += dx * follow_speed.min(1.0);
        self.current_position.y += dy * follow_speed.min(1.0);

        // Zoom control
        if ctx.input.is_key_pressed(KeyCode::Equal) {
            self.zoom_change = 1;
        } else if ctx.input.is_key_pressed(KeyCode::Minus) {
            self.zoom_change = -1;
        }
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // CameraController doesn't render
    }

    fn on_key_down(&mut self, _ctx: &InputContext, key: KeyCode) {
        match key {
            KeyCode::Equal => self.zoom_change = 1,
            KeyCode::Minus => self.zoom_change = -1,
            _ => {}
        }
    }
}
