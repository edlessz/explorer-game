use crate::platform::Platform;

pub struct Game<P: Platform> {
    pixel_buffer: Vec<u32>,
    width: usize,
    height: usize,
    platform: P,
}

impl<P: Platform> Game<P> {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        let buffer_size = width * height;
        let platform = P::new(width, height, title);

        Self {
            pixel_buffer: vec![0; buffer_size],
            width,
            height,
            platform,
        }
    }

    pub fn run(&mut self) -> ! {
        // draw red square in the center
        let square_size = 100;
        let start_x = (self.width - square_size) / 2;
        let start_y = (self.height - square_size) / 2;
        for y in start_y..start_y + square_size {
            for x in start_x..start_x + square_size {
                let index = y * self.width + x;
                self.pixel_buffer[index] = 0xFF0000; // Red color in ARGB
            }
        }

        loop {
            if !self.platform.update(&self.pixel_buffer) {
                std::process::exit(0);
            }
        }
    }

    // For WASM: initialize and render once without the blocking loop
    pub fn init_and_render(&mut self) {
        // draw red square in the center
        let square_size = 100;
        let start_x = (self.width - square_size) / 2;
        let start_y = (self.height - square_size) / 2;
        for y in start_y..start_y + square_size {
            for x in start_x..start_x + square_size {
                let index = y * self.width + x;
                self.pixel_buffer[index] = 0xFF0000; // Red color in ARGB
            }
        }

        // Render once
        self.platform.update(&self.pixel_buffer);
    }
}
