use crate::types::Vec2;

/// ARGB color format: 0xAARRGGBB
pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    rgba(r, g, b, 255)
}

#[derive(Clone, Debug)]
struct TransformState {
    translate: Vec2,
    scale: Vec2,
    rotation: f32,
}

impl Default for TransformState {
    fn default() -> Self {
        Self {
            translate: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
        }
    }
}

pub struct Renderer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    transform_stack: Vec<TransformState>,
    current_transform: TransformState,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
            transform_stack: Vec::new(),
            current_transform: TransformState::default(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    /// Set a single pixel (in screen coordinates)
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = color;
        }
    }

    /// Get a single pixel (in screen coordinates)
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<u32> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as usize) * self.width + (x as usize);
            Some(self.buffer[index])
        } else {
            None
        }
    }

    // Transform operations
    pub fn push_transform(&mut self) {
        self.transform_stack.push(self.current_transform.clone());
    }

    pub fn pop_transform(&mut self) {
        if let Some(t) = self.transform_stack.pop() {
            self.current_transform = t;
        }
    }

    pub fn reset_transform(&mut self) {
        self.current_transform = TransformState::default();
        self.transform_stack.clear();
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        // Apply translation in current coordinate system
        let cos = self.current_transform.rotation.cos();
        let sin = self.current_transform.rotation.sin();
        let scaled_x = x * self.current_transform.scale.x;
        let scaled_y = y * self.current_transform.scale.y;
        self.current_transform.translate.x += scaled_x * cos - scaled_y * sin;
        self.current_transform.translate.y += scaled_x * sin + scaled_y * cos;
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.current_transform.scale.x *= sx;
        self.current_transform.scale.y *= sy;
    }

    pub fn rotate(&mut self, angle: f32) {
        self.current_transform.rotation += angle;
    }

    /// Transform a world coordinate to screen coordinate
    fn world_to_screen(&self, x: f32, y: f32) -> (i32, i32) {
        let cos = self.current_transform.rotation.cos();
        let sin = self.current_transform.rotation.sin();

        // Apply scale and rotation
        let scaled_x = x * self.current_transform.scale.x;
        let scaled_y = y * self.current_transform.scale.y;
        let rotated_x = scaled_x * cos - scaled_y * sin;
        let rotated_y = scaled_x * sin + scaled_y * cos;

        // Apply translation
        let screen_x = rotated_x + self.current_transform.translate.x;
        let screen_y = rotated_y + self.current_transform.translate.y;

        (screen_x.round() as i32, screen_y.round() as i32)
    }

    /// Fill a rectangle in world coordinates (affected by current transform)
    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: u32) {
        // For axis-aligned case (no rotation), use fast path
        if self.current_transform.rotation.abs() < 0.001 {
            let (sx, sy) = self.world_to_screen(x, y);
            let sw = (w * self.current_transform.scale.x).round() as i32;
            let sh = (h * self.current_transform.scale.y).round() as i32;

            let x0 = sx.max(0);
            let y0 = sy.max(0);
            let x1 = (sx + sw).min(self.width as i32);
            let y1 = (sy + sh).min(self.height as i32);

            for py in y0..y1 {
                for px in x0..x1 {
                    let index = (py as usize) * self.width + (px as usize);
                    self.buffer[index] = color;
                }
            }
        } else {
            // Rotated rectangle - sample corners and fill
            self.fill_rotated_rect(x, y, w, h, color);
        }
    }

    fn fill_rotated_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: u32) {
        // Get the four corners in screen space
        let corners = [
            self.world_to_screen(x, y),
            self.world_to_screen(x + w, y),
            self.world_to_screen(x + w, y + h),
            self.world_to_screen(x, y + h),
        ];

        // Find bounding box
        let min_x = corners.iter().map(|c| c.0).min().unwrap_or(0);
        let max_x = corners.iter().map(|c| c.0).max().unwrap_or(0);
        let min_y = corners.iter().map(|c| c.1).min().unwrap_or(0);
        let max_y = corners.iter().map(|c| c.1).max().unwrap_or(0);

        // Clip to screen
        let min_x = min_x.max(0);
        let min_y = min_y.max(0);
        let max_x = max_x.min(self.width as i32 - 1);
        let max_y = max_y.min(self.height as i32 - 1);

        // For each pixel in bounding box, check if inside polygon
        for py in min_y..=max_y {
            for px in min_x..=max_x {
                if self.point_in_polygon(px, py, &corners) {
                    let index = (py as usize) * self.width + (px as usize);
                    self.buffer[index] = color;
                }
            }
        }
    }

    fn point_in_polygon(&self, px: i32, py: i32, corners: &[(i32, i32); 4]) -> bool {
        let mut inside = false;
        let n = corners.len();
        let mut j = n - 1;

        for i in 0..n {
            let (xi, yi) = corners[i];
            let (xj, yj) = corners[j];

            if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    /// Fill a rectangle in screen coordinates (ignores transform)
    pub fn fill_rect_screen(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let x0 = x.max(0);
        let y0 = y.max(0);
        let x1 = (x + w).min(self.width as i32);
        let y1 = (y + h).min(self.height as i32);

        for py in y0..y1 {
            for px in x0..x1 {
                let index = (py as usize) * self.width + (px as usize);
                self.buffer[index] = color;
            }
        }
    }

    /// Draw a horizontal line (screen coordinates)
    pub fn draw_hline(&mut self, x0: i32, x1: i32, y: i32, color: u32) {
        if y < 0 || y >= self.height as i32 {
            return;
        }
        let x_start = x0.max(0) as usize;
        let x_end = (x1.min(self.width as i32)) as usize;
        let y = y as usize;

        for x in x_start..x_end {
            self.buffer[y * self.width + x] = color;
        }
    }

    /// Draw a vertical line (screen coordinates)
    pub fn draw_vline(&mut self, x: i32, y0: i32, y1: i32, color: u32) {
        if x < 0 || x >= self.width as i32 {
            return;
        }
        let y_start = y0.max(0) as usize;
        let y_end = (y1.min(self.height as i32)) as usize;
        let x = x as usize;

        for y in y_start..y_end {
            self.buffer[y * self.width + x] = color;
        }
    }

    /// Blit a source buffer onto this renderer at the given screen position
    pub fn blit(
        &mut self,
        src: &[u32],
        src_width: usize,
        src_height: usize,
        dst_x: i32,
        dst_y: i32,
    ) {
        for sy in 0..src_height {
            let dy = dst_y + sy as i32;
            if dy < 0 || dy >= self.height as i32 {
                continue;
            }
            for sx in 0..src_width {
                let dx = dst_x + sx as i32;
                if dx < 0 || dx >= self.width as i32 {
                    continue;
                }
                let src_idx = sy * src_width + sx;
                let dst_idx = (dy as usize) * self.width + (dx as usize);
                self.buffer[dst_idx] = src[src_idx];
            }
        }
    }

    /// Blit with alpha blending
    pub fn blit_with_alpha(
        &mut self,
        src: &[u32],
        src_width: usize,
        src_height: usize,
        dst_x: i32,
        dst_y: i32,
    ) {
        for sy in 0..src_height {
            let dy = dst_y + sy as i32;
            if dy < 0 || dy >= self.height as i32 {
                continue;
            }
            for sx in 0..src_width {
                let dx = dst_x + sx as i32;
                if dx < 0 || dx >= self.width as i32 {
                    continue;
                }
                let src_idx = sy * src_width + sx;
                let dst_idx = (dy as usize) * self.width + (dx as usize);

                let src_color = src[src_idx];
                let src_a = ((src_color >> 24) & 0xFF) as u32;

                if src_a == 255 {
                    self.buffer[dst_idx] = src_color;
                } else if src_a > 0 {
                    let dst_color = self.buffer[dst_idx];
                    self.buffer[dst_idx] = blend_colors(dst_color, src_color);
                }
            }
        }
    }

    /// Apply a lighting multiplier to a region (for light map overlay)
    pub fn apply_lighting(&mut self, x: i32, y: i32, w: i32, h: i32, light: f32) {
        let x0 = x.max(0);
        let y0 = y.max(0);
        let x1 = (x + w).min(self.width as i32);
        let y1 = (y + h).min(self.height as i32);

        let light = light.clamp(0.0, 1.0);

        for py in y0..y1 {
            for px in x0..x1 {
                let index = (py as usize) * self.width + (px as usize);
                let color = self.buffer[index];

                let r = ((color >> 16) & 0xFF) as f32 * light;
                let g = ((color >> 8) & 0xFF) as f32 * light;
                let b = (color & 0xFF) as f32 * light;

                self.buffer[index] = rgb(r as u8, g as u8, b as u8);
            }
        }
    }

    /// Get current transform scale (useful for camera)
    pub fn current_scale(&self) -> Vec2 {
        self.current_transform.scale
    }

    /// Get current transform translation
    pub fn current_translation(&self) -> Vec2 {
        self.current_transform.translate
    }
}

fn blend_colors(dst: u32, src: u32) -> u32 {
    let src_a = ((src >> 24) & 0xFF) as u32;
    let src_r = ((src >> 16) & 0xFF) as u32;
    let src_g = ((src >> 8) & 0xFF) as u32;
    let src_b = (src & 0xFF) as u32;

    let dst_r = ((dst >> 16) & 0xFF) as u32;
    let dst_g = ((dst >> 8) & 0xFF) as u32;
    let dst_b = (dst & 0xFF) as u32;

    let inv_a = 255 - src_a;

    let r = (src_r * src_a + dst_r * inv_a) / 255;
    let g = (src_g * src_a + dst_g * inv_a) / 255;
    let b = (src_b * src_a + dst_b * inv_a) / 255;

    rgb(r as u8, g as u8, b as u8)
}
