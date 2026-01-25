use crate::types::Vec2;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum KeyCode {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Space,
    Enter,
    Escape,
    Equal,
    Minus,
    KeyW,
    KeyA,
    KeyS,
    KeyD,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Unknown,
}

impl KeyCode {
    /// Convert from a string key name (as used in JavaScript KeyboardEvent.key)
    pub fn from_str(s: &str) -> Self {
        match s {
            "ArrowUp" | "Up" => KeyCode::ArrowUp,
            "ArrowDown" | "Down" => KeyCode::ArrowDown,
            "ArrowLeft" | "Left" => KeyCode::ArrowLeft,
            "ArrowRight" | "Right" => KeyCode::ArrowRight,
            " " | "Space" => KeyCode::Space,
            "Enter" => KeyCode::Enter,
            "Escape" | "Esc" => KeyCode::Escape,
            "=" | "Equal" => KeyCode::Equal,
            "-" | "Minus" => KeyCode::Minus,
            "w" | "W" => KeyCode::KeyW,
            "a" | "A" => KeyCode::KeyA,
            "s" | "S" => KeyCode::KeyS,
            "d" | "D" => KeyCode::KeyD,
            "0" => KeyCode::Digit0,
            "1" => KeyCode::Digit1,
            "2" => KeyCode::Digit2,
            "3" => KeyCode::Digit3,
            "4" => KeyCode::Digit4,
            "5" => KeyCode::Digit5,
            "6" => KeyCode::Digit6,
            "7" => KeyCode::Digit7,
            "8" => KeyCode::Digit8,
            "9" => KeyCode::Digit9,
            _ => KeyCode::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
}

impl MouseButton {
    pub fn from_button_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(MouseButton::Left),
            1 => Some(MouseButton::Middle),
            2 => Some(MouseButton::Right),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum InputEvent {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    MouseDown(MouseButton, Vec2),
    MouseUp(MouseButton, Vec2),
    MouseMove(Vec2),
}

#[derive(Clone, Debug, Default)]
pub struct InputState {
    pressed_keys: HashSet<KeyCode>,
    pressed_mouse_buttons: HashSet<MouseButton>,
    mouse_position: Vec2,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_event(&mut self, event: &InputEvent) {
        match event {
            InputEvent::KeyDown(key) => {
                self.pressed_keys.insert(*key);
            }
            InputEvent::KeyUp(key) => {
                self.pressed_keys.remove(key);
            }
            InputEvent::MouseDown(button, pos) => {
                self.pressed_mouse_buttons.insert(*button);
                self.mouse_position = *pos;
            }
            InputEvent::MouseUp(button, pos) => {
                self.pressed_mouse_buttons.remove(button);
                self.mouse_position = *pos;
            }
            InputEvent::MouseMove(pos) => {
                self.mouse_position = *pos;
            }
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.pressed_mouse_buttons.clear();
    }
}
