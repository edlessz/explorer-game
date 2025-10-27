use minifb::{Window, WindowOptions};

pub struct Game {
    pixel_buffer: Vec<u8>,
    window_buffer: Vec<u32>,
    width: usize,
    height: usize,
    window: Window,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        // 3 bytes per pixel (RGB)
        let buffer_size = width * height * 3;

        let mut window = Window::new(
            "Explorer Game",
            width,
            height,
            WindowOptions::default(),
        )
        .expect("Unable to create window");

        // Limit to max ~60 fps update rate
        window.set_target_fps(60);

        Self {
            pixel_buffer: vec![0; buffer_size],
            window_buffer: vec![0; width * height],
            width,
            height,
            window,
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            if !self.window.is_open() || self.window.is_key_down(minifb::Key::Escape) {
                std::process::exit(0);
            }

            // Convert RGB buffer to u32 buffer for minifb
            for i in 0..(self.width * self.height) {
                let rgb_index = i * 3;
                let r = self.pixel_buffer[rgb_index] as u32;
                let g = self.pixel_buffer[rgb_index + 1] as u32;
                let b = self.pixel_buffer[rgb_index + 2] as u32;
                self.window_buffer[i] = (r << 16) | (g << 8) | b;
            }

            self.window
                .update_with_buffer(&self.window_buffer, self.width, self.height)
                .expect("Failed to update window");
        }
    }
}
