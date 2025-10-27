use crate::renderer::Renderer;

pub struct Game<R: Renderer> {
    pixel_buffer: Vec<u32>,
    width: usize,
    height: usize,
    renderer: R,
}

impl<R: Renderer> Game<R> {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        let buffer_size = width * height;
        let renderer = R::new(width, height, title);

    
        Self {
            pixel_buffer: vec![0; buffer_size],
            width,
            height,
            renderer,
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
            if !self.renderer.update(&self.pixel_buffer) {
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
        self.renderer.update(&self.pixel_buffer);
    }
}
