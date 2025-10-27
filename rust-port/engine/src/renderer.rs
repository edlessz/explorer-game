pub trait Renderer {
    fn new(width: usize, height: usize, title: &str) -> Self
    where
        Self: Sized;
    fn update(&mut self, buffer: &[u32]) -> bool; // Returns false if window should close
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}
