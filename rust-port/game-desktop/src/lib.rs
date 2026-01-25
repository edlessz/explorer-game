use engine::input::{InputEvent, KeyCode, MouseButton};
use engine::types::Vec2;
use engine::Platform;
use minifb::{Key, MouseButton as MinifbMouseButton, MouseMode, Window, WindowOptions};
use std::collections::HashSet;

pub struct DesktopPlatform {
    window: Window,
    width: usize,
    height: usize,
    pressed_keys: HashSet<KeyCode>,
    pressed_mouse_buttons: HashSet<MouseButton>,
    mouse_position: Vec2,
    pending_events: Vec<InputEvent>,
}

impl DesktopPlatform {
    fn minifb_key_to_keycode(key: Key) -> Option<KeyCode> {
        match key {
            Key::Up => Some(KeyCode::ArrowUp),
            Key::Down => Some(KeyCode::ArrowDown),
            Key::Left => Some(KeyCode::ArrowLeft),
            Key::Right => Some(KeyCode::ArrowRight),
            Key::Space => Some(KeyCode::Space),
            Key::Enter => Some(KeyCode::Enter),
            Key::Escape => Some(KeyCode::Escape),
            Key::Equal => Some(KeyCode::Equal),
            Key::Minus => Some(KeyCode::Minus),
            Key::W => Some(KeyCode::KeyW),
            Key::A => Some(KeyCode::KeyA),
            Key::S => Some(KeyCode::KeyS),
            Key::D => Some(KeyCode::KeyD),
            Key::Key0 => Some(KeyCode::Digit0),
            Key::Key1 => Some(KeyCode::Digit1),
            Key::Key2 => Some(KeyCode::Digit2),
            Key::Key3 => Some(KeyCode::Digit3),
            Key::Key4 => Some(KeyCode::Digit4),
            Key::Key5 => Some(KeyCode::Digit5),
            Key::Key6 => Some(KeyCode::Digit6),
            Key::Key7 => Some(KeyCode::Digit7),
            Key::Key8 => Some(KeyCode::Digit8),
            Key::Key9 => Some(KeyCode::Digit9),
            _ => None,
        }
    }

    fn minifb_mouse_to_button(button: MinifbMouseButton) -> Option<MouseButton> {
        match button {
            MinifbMouseButton::Left => Some(MouseButton::Left),
            MinifbMouseButton::Middle => Some(MouseButton::Middle),
            MinifbMouseButton::Right => Some(MouseButton::Right),
        }
    }

    fn poll_keyboard(&mut self) {
        // Check all keys we care about
        let keys_to_check = [
            Key::Up,
            Key::Down,
            Key::Left,
            Key::Right,
            Key::Space,
            Key::Enter,
            Key::Escape,
            Key::Equal,
            Key::Minus,
            Key::W,
            Key::A,
            Key::S,
            Key::D,
            Key::Key0,
            Key::Key1,
            Key::Key2,
            Key::Key3,
            Key::Key4,
            Key::Key5,
            Key::Key6,
            Key::Key7,
            Key::Key8,
            Key::Key9,
        ];

        for key in keys_to_check {
            if let Some(keycode) = Self::minifb_key_to_keycode(key) {
                let is_down = self.window.is_key_down(key);
                let was_pressed = self.pressed_keys.contains(&keycode);

                if is_down && !was_pressed {
                    self.pressed_keys.insert(keycode);
                    self.pending_events.push(InputEvent::KeyDown(keycode));
                } else if !is_down && was_pressed {
                    self.pressed_keys.remove(&keycode);
                    self.pending_events.push(InputEvent::KeyUp(keycode));
                }
            }
        }
    }

    fn poll_mouse(&mut self) {
        // Get mouse position
        if let Some((x, y)) = self.window.get_mouse_pos(MouseMode::Clamp) {
            let new_pos = Vec2::new(x, y);
            if (new_pos.x - self.mouse_position.x).abs() > 0.1
                || (new_pos.y - self.mouse_position.y).abs() > 0.1
            {
                self.mouse_position = new_pos;
                self.pending_events
                    .push(InputEvent::MouseMove(self.mouse_position));
            }
        }

        // Check mouse buttons
        let buttons_to_check = [
            MinifbMouseButton::Left,
            MinifbMouseButton::Middle,
            MinifbMouseButton::Right,
        ];

        for button in buttons_to_check {
            if let Some(mb) = Self::minifb_mouse_to_button(button) {
                let is_down = self.window.get_mouse_down(button);
                let was_pressed = self.pressed_mouse_buttons.contains(&mb);

                if is_down && !was_pressed {
                    self.pressed_mouse_buttons.insert(mb);
                    self.pending_events
                        .push(InputEvent::MouseDown(mb, self.mouse_position));
                } else if !is_down && was_pressed {
                    self.pressed_mouse_buttons.remove(&mb);
                    self.pending_events
                        .push(InputEvent::MouseUp(mb, self.mouse_position));
                }
            }
        }
    }
}

impl Platform for DesktopPlatform {
    fn new(width: usize, height: usize, title: &str) -> Self {
        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                ..Default::default()
            },
        )
        .expect("Unable to create window");

        window.set_target_fps(60);

        Self {
            window,
            width,
            height,
            pressed_keys: HashSet::new(),
            pressed_mouse_buttons: HashSet::new(),
            mouse_position: Vec2::ZERO,
            pending_events: Vec::new(),
        }
    }

    fn update(&mut self, buffer: &[u32]) -> bool {
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            return false;
        }

        self.window
            .update_with_buffer(buffer, self.width, self.height)
            .expect("Failed to update window");

        true
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        self.pending_events.clear();

        self.poll_keyboard();
        self.poll_mouse();

        std::mem::take(&mut self.pending_events)
    }

    fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
}
