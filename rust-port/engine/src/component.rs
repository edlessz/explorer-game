use crate::input::{InputEvent, KeyCode, MouseButton};
use crate::renderer::Renderer;
use crate::types::Vec2;
use std::any::Any;

/// Context passed to components during setup
pub struct SetupContext<'a> {
    pub game_width: usize,
    pub game_height: usize,
    pub entity_tag: Option<&'a str>,
}

/// Context passed to components during update
pub struct UpdateContext<'a> {
    pub delta_time: f32,
    pub input: &'a dyn InputProvider,
    pub game_width: usize,
    pub game_height: usize,
}

/// Context passed to components during render
pub struct RenderContext<'a> {
    pub renderer: &'a mut Renderer,
    pub camera_bounds: Option<(Vec2, Vec2)>,
    pub ppu: Vec2,
}

/// Context passed to components during input events
pub struct InputContext<'a> {
    pub input: &'a dyn InputProvider,
}

/// Trait for querying input state
pub trait InputProvider {
    fn is_key_pressed(&self, key: KeyCode) -> bool;
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool;
    fn mouse_position(&self) -> Vec2;
}

/// General component context for inter-component communication
pub struct ComponentContext<'a> {
    pub entity_index: usize,
    pub components: &'a mut [Box<dyn Component>],
}

impl<'a> ComponentContext<'a> {
    pub fn get_sibling<T: Component + 'static>(&self) -> Option<&T> {
        self.components.iter().find_map(|c| c.as_any().downcast_ref::<T>())
    }

    pub fn get_sibling_mut<T: Component + 'static>(&mut self) -> Option<&mut T> {
        self.components
            .iter_mut()
            .find_map(|c| c.as_any_mut().downcast_mut::<T>())
    }
}

/// The Component trait that all game components implement
pub trait Component: Any + Send + Sync {
    /// Return self as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Return self as mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Component name for serialization/debugging
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Whether this component is enabled
    fn enabled(&self) -> bool {
        true
    }

    /// Called once when the entity is added to the game
    fn setup(&mut self, _ctx: &SetupContext) {}

    /// Called every frame
    fn update(&mut self, _ctx: &UpdateContext) {}

    /// Called every frame for rendering
    fn render(&self, _ctx: &mut RenderContext) {}

    /// Called when a key is pressed
    fn on_key_down(&mut self, _ctx: &InputContext, _key: KeyCode) {}

    /// Called when a key is released
    fn on_key_up(&mut self, _ctx: &InputContext, _key: KeyCode) {}

    /// Called when a mouse button is pressed
    fn on_mouse_down(&mut self, _ctx: &InputContext, _button: MouseButton, _pos: Vec2) {}

    /// Called when a mouse button is released
    fn on_mouse_up(&mut self, _ctx: &InputContext, _button: MouseButton, _pos: Vec2) {}

    /// Called when the mouse moves
    fn on_mouse_move(&mut self, _ctx: &InputContext, _pos: Vec2) {}
}

/// Macro to implement the as_any methods for a component
#[macro_export]
macro_rules! impl_component_any {
    ($type:ty) => {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    };
}
