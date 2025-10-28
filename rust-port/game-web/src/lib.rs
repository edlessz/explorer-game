use engine::{Game, Platform};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub struct WebPlatform {
    context: CanvasRenderingContext2d,
    width: usize,
    height: usize,
}

impl Platform for WebPlatform {
    fn new(width: usize, height: usize, _title: &str) -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        // Get canvas with id "viewport"
        let canvas = document
            .get_element_by_id("viewport")
            .expect("should find canvas with id 'viewport'")
            .dyn_into::<HtmlCanvasElement>()
            .expect("element should be HtmlCanvasElement");

        // Set canvas size
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let context = canvas
            .get_context("2d")
            .expect("should get 2d context")
            .expect("should unwrap 2d context")
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("should cast to CanvasRenderingContext2d");

        Self {
            context,
            width,
            height,
        }
    }

    fn update(&mut self, buffer: &[u32]) -> bool {
        // Convert u32 ARGB buffer to RGBA bytes for ImageData
        let mut rgba_buffer: Vec<u8> = Vec::with_capacity(buffer.len() * 4);

        for &pixel in buffer {
            // Extract ARGB components
            let a = ((pixel >> 24) & 0xFF) as u8;
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;

            // Push as RGBA
            rgba_buffer.push(r);
            rgba_buffer.push(g);
            rgba_buffer.push(b);
            rgba_buffer.push(if a == 0 { 255 } else { a }); // Default to opaque if alpha is 0
        }

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&rgba_buffer),
            self.width as u32,
            self.height as u32,
        )
        .expect("should create ImageData");

        self.context
            .put_image_data(&image_data, 0.0, 0.0)
            .expect("should put image data");

        // Return true to keep running (WASM apps don't typically close)
        true
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let width: usize = 800;
    let height: usize = 600;

    let mut game = Game::<WebPlatform>::new(width, height, "Explorer Game");

    // Initialize and render once (no infinite loop for WASM)
    game.init_and_render();

    Ok(())
}
