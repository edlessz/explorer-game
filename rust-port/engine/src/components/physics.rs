use crate::component::{Component, RenderContext, SetupContext, UpdateContext};
use crate::impl_component_any;
use crate::types::{Transform, Vec2};

/// Default gravity value (units per second squared)
pub const DEFAULT_GRAVITY: f32 = 20.0;

/// Physics component for velocity-based movement with gravity
pub struct Physics {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub gravity_enabled: bool,
    /// Reference to the entity's transform (updated externally)
    transform: Transform,
    /// Collision callback - set by game to handle collision response
    collision_check: Option<Box<dyn Fn(&Transform) -> bool + Send + Sync>>,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            acceleration: Vec2::new(0.0, DEFAULT_GRAVITY),
            gravity_enabled: true,
            transform: Transform::default(),
            collision_check: None,
        }
    }
}

impl Physics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gravity(mut self, enabled: bool) -> Self {
        self.gravity_enabled = enabled;
        if !enabled {
            self.acceleration.y = 0.0;
        }
        self
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    /// Get the updated transform after physics simulation
    pub fn get_updated_transform(&self) -> Transform {
        self.transform.clone()
    }

    /// Simulate one physics step and return the new position
    /// Returns (new_position, velocity_after_collision)
    pub fn simulate_step(
        &mut self,
        delta_time: f32,
        check_collision: impl Fn(f32, f32) -> bool,
    ) -> (Vec2, Vec2) {
        let mut pos = self.transform.position.xy();
        let mut vel = self.velocity;

        // X-axis movement
        vel.x += self.acceleration.x * delta_time;
        let new_x = pos.x + vel.x * delta_time;

        // Check X collision
        if check_collision(new_x, pos.y) {
            // Move back until not colliding
            let sign = if vel.x > 0.0 { 1.0 } else { -1.0 };
            let mut test_x = new_x;
            while check_collision(test_x, pos.y) {
                test_x -= sign * 0.001;
            }
            pos.x = test_x;
            vel.x = 0.0;
        } else {
            pos.x = new_x;
        }

        // Y-axis movement
        vel.y += self.acceleration.y * delta_time;
        let new_y = pos.y + vel.y * delta_time;

        // Check Y collision
        if check_collision(pos.x, new_y) {
            // Move back until not colliding
            let sign = if vel.y > 0.0 { 1.0 } else { -1.0 };
            let mut test_y = new_y;
            while check_collision(pos.x, test_y) {
                test_y -= sign * 0.001;
            }
            pos.y = test_y;
            vel.y = 0.0;
        } else {
            pos.y = new_y;
        }

        self.velocity = vel;
        (pos, vel)
    }
}

impl Component for Physics {
    impl_component_any!(Physics);

    fn name(&self) -> &'static str {
        "Physics"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, _ctx: &UpdateContext) {
        // Physics update is handled by the game loop with collision checking
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // Physics doesn't render
    }
}
