use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::input::KeyCode;
use crate::types::{Transform, Vec2};

const DEFAULT_MOVE_SPEED: f32 = 10.0;
const DEFAULT_JUMP_HEIGHT: f32 = 10.0;

/// Player controller component for movement and jumping
pub struct PlayerController {
    pub move_speed: f32,
    pub jump_height: f32,
    /// Current movement direction (-1, 0, or 1)
    direction: f32,
    /// Whether jump was requested this frame
    jump_requested: bool,
    /// Velocity to apply (set by update, used by game physics)
    pub velocity: Vec2,
    /// Is the player grounded (set externally by physics system)
    pub grounded: bool,
    /// Entity transform reference
    transform: Transform,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            move_speed: DEFAULT_MOVE_SPEED,
            jump_height: DEFAULT_JUMP_HEIGHT,
            direction: 0.0,
            jump_requested: false,
            velocity: Vec2::ZERO,
            grounded: false,
            transform: Transform::default(),
        }
    }
}

impl PlayerController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.move_speed = speed;
        self
    }

    pub fn with_jump_height(mut self, height: f32) -> Self {
        self.jump_height = height;
        self
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }

    pub fn set_grounded(&mut self, grounded: bool) {
        self.grounded = grounded;
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }
}

impl Component for PlayerController {
    impl_component_any!(PlayerController);

    fn name(&self) -> &'static str {
        "PlayerController"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, ctx: &UpdateContext) {
        // Calculate movement direction
        let right = if ctx.input.is_key_pressed(KeyCode::ArrowRight) || ctx.input.is_key_pressed(KeyCode::KeyD) {
            1.0
        } else {
            0.0
        };
        let left = if ctx.input.is_key_pressed(KeyCode::ArrowLeft) || ctx.input.is_key_pressed(KeyCode::KeyA) {
            1.0
        } else {
            0.0
        };

        self.direction = right - left;
        self.velocity.x = self.direction * self.move_speed;

        // Jump if grounded and jump key pressed
        if (ctx.input.is_key_pressed(KeyCode::ArrowUp) || ctx.input.is_key_pressed(KeyCode::KeyW) || ctx.input.is_key_pressed(KeyCode::Space))
            && self.grounded
        {
            self.velocity.y = -self.jump_height;
        }
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // PlayerController doesn't render
    }
}
