const FPS_SAMPLE_COUNT: usize = 60;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(not(target_arch = "wasm32"))]
pub struct FrameTimer {
    last_time: Instant,
    fps_samples: Vec<f32>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameTimer {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            fps_samples: Vec::with_capacity(FPS_SAMPLE_COUNT),
        }
    }

    /// Records a frame and returns the delta time in seconds since the last frame
    pub fn record_frame(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_time);
        self.last_time = now;

        let delta_secs = delta.as_secs_f32();

        // Avoid division by zero
        if delta_secs > 0.0 {
            let fps = 1.0 / delta_secs;
            self.fps_samples.push(fps);
            if self.fps_samples.len() > FPS_SAMPLE_COUNT {
                self.fps_samples.remove(0);
            }
        }

        delta_secs
    }

    /// Returns the average FPS over the last N frames
    pub fn get_fps(&self) -> f32 {
        if self.fps_samples.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.fps_samples.iter().sum();
        sum / self.fps_samples.len() as f32
    }

    /// Reset the timer (useful when resuming from pause)
    pub fn reset(&mut self) {
        self.last_time = Instant::now();
        self.fps_samples.clear();
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for FrameTimer {
    fn default() -> Self {
        Self::new()
    }
}

// WASM implementation using performance.now()
#[cfg(target_arch = "wasm32")]
pub struct FrameTimer {
    last_time: f64,
    fps_samples: Vec<f32>,
}

#[cfg(target_arch = "wasm32")]
fn performance_now() -> f64 {
    web_sys::window()
        .expect("no window")
        .performance()
        .expect("no performance")
        .now()
}

#[cfg(target_arch = "wasm32")]
impl FrameTimer {
    pub fn new() -> Self {
        Self {
            last_time: performance_now(),
            fps_samples: Vec::with_capacity(FPS_SAMPLE_COUNT),
        }
    }

    /// Records a frame and returns the delta time in seconds since the last frame
    pub fn record_frame(&mut self) -> f32 {
        let now = performance_now();
        let delta_ms = now - self.last_time;
        self.last_time = now;

        let delta_secs = (delta_ms / 1000.0) as f32;

        // Avoid division by zero
        if delta_secs > 0.0 {
            let fps = 1.0 / delta_secs;
            self.fps_samples.push(fps);
            if self.fps_samples.len() > FPS_SAMPLE_COUNT {
                self.fps_samples.remove(0);
            }
        }

        delta_secs
    }

    /// Returns the average FPS over the last N frames
    pub fn get_fps(&self) -> f32 {
        if self.fps_samples.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.fps_samples.iter().sum();
        sum / self.fps_samples.len() as f32
    }

    /// Reset the timer (useful when resuming from pause)
    pub fn reset(&mut self) {
        self.last_time = performance_now();
        self.fps_samples.clear();
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for FrameTimer {
    fn default() -> Self {
        Self::new()
    }
}
