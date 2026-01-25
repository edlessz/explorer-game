use crate::input::{InputEvent, KeyCode, MouseButton};
use crate::types::Vec2;

pub trait Platform {
    fn new(width: usize, height: usize, title: &str) -> Self
    where
        Self: Sized;

    // Rendering
    fn update(&mut self, buffer: &[u32]) -> bool; // Returns false if window should close
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    // Input - poll for events that occurred since last call
    fn poll_events(&mut self) -> Vec<InputEvent>;

    // Input state queries
    fn is_key_pressed(&self, key: KeyCode) -> bool;
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool;
    fn mouse_position(&self) -> Vec2;
}
