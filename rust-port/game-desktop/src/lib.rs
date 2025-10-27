use engine::Renderer;
use minifb::{Key, Window, WindowOptions};

pub struct MiniFbRenderer {
    window: Window,
    width: usize,
    height: usize,
}

impl Renderer for MiniFbRenderer {
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
}
