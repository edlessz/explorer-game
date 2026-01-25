use engine::component::{Component, InputContext, RenderContext, SetupContext, UpdateContext};
use engine::impl_component_any;
use engine::input::MouseButton;
use engine::types::{Transform, Vec2};

/// Cursor controller for mouse-based tile editing
pub struct CursorController {
    /// Screen position of cursor
    pub screen_position: Vec2,
    /// World position of cursor (converted from screen)
    pub world_position: Vec2,
    /// Tile to place on left click (0 = erase)
    pub place_tile_id: u8,
    /// Whether left mouse is held
    left_held: bool,
    /// Whether right mouse is held
    right_held: bool,
    /// Pending tile operations: (x, y, tile_id)
    pub pending_operations: Vec<(i32, i32, u8)>,
    /// Entity transform
    transform: Transform,
}

impl Default for CursorController {
    fn default() -> Self {
        Self {
            screen_position: Vec2::ZERO,
            world_position: Vec2::ZERO,
            place_tile_id: 4, // Light tile by default
            left_held: false,
            right_held: false,
            pending_operations: Vec::new(),
            transform: Transform::default(),
        }
    }
}

impl CursorController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }

    pub fn set_world_position(&mut self, pos: Vec2) {
        self.world_position = pos;
        // Update entity transform to cursor world position
        self.transform.position.x = pos.x.floor();
        self.transform.position.y = pos.y.floor();
    }

    pub fn get_tile_position(&self) -> (i32, i32) {
        (
            self.world_position.x.floor() as i32,
            self.world_position.y.floor() as i32,
        )
    }

    /// Take pending tile operations
    pub fn take_pending_operations(&mut self) -> Vec<(i32, i32, u8)> {
        std::mem::take(&mut self.pending_operations)
    }

    fn handle_click(&mut self, is_left: bool) {
        let (x, y) = self.get_tile_position();
        let tile_id = if is_left { self.place_tile_id } else { 0 };
        self.pending_operations.push((x, y, tile_id));
    }
}

impl Component for CursorController {
    impl_component_any!(CursorController);

    fn name(&self) -> &'static str {
        "CursorController"
    }

    fn setup(&mut self, _ctx: &SetupContext) {}

    fn update(&mut self, ctx: &UpdateContext) {
        // Update screen position from input
        self.screen_position = ctx.input.mouse_position();

        // Handle held mouse buttons for continuous placing/erasing
        if self.left_held || ctx.input.is_mouse_button_pressed(MouseButton::Left) {
            self.handle_click(true);
        }
        if self.right_held || ctx.input.is_mouse_button_pressed(MouseButton::Right) {
            self.handle_click(false);
        }
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // Cursor rendering is handled by ColorRenderer on the same entity
    }

    fn on_mouse_down(&mut self, _ctx: &InputContext, button: MouseButton, _pos: Vec2) {
        match button {
            MouseButton::Left => {
                self.left_held = true;
                self.handle_click(true);
            }
            MouseButton::Right => {
                self.right_held = true;
                self.handle_click(false);
            }
            _ => {}
        }
    }

    fn on_mouse_up(&mut self, _ctx: &InputContext, button: MouseButton, _pos: Vec2) {
        match button {
            MouseButton::Left => self.left_held = false,
            MouseButton::Right => self.right_held = false,
            _ => {}
        }
    }

    fn on_mouse_move(&mut self, _ctx: &InputContext, pos: Vec2) {
        self.screen_position = pos;
    }
}
