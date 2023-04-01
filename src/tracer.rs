use std::time::{Duration, Instant};

pub struct Tracer {
    width: usize,
    height: usize,
    image_buffer: Vec<u8>,
    frame_time: Duration,
}

impl Tracer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            image_buffer: vec![255; width * height * 3],
            frame_time: Duration::default(),
        }
    }

    pub fn trace(&mut self) {
        let frame_time = Instant::now();

        self.draw_gradient();

        self.frame_time = frame_time.elapsed();
    }

    fn draw_gradient(&mut self) {
        for i in 0..self.image_buffer.len() {
            let y = i / self.width * 3;
            let t = self.height / (y + 1);
            self.image_buffer[i] = crate::math::lerp(0, 255, t as f32);
        }
    }

    pub fn image_buffer(&self) -> &[u8] {
        &self.image_buffer
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn resize(&mut self, width: usize, height: usize) -> anyhow::Result<()> {
        if width < 1 || height < 1 {
            anyhow::bail!("Failed to resize tracer; size must be greater than zero!")
        }

        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.image_buffer.resize(width * height * 3, 255);
        }
        Ok(())
    }

    pub fn frame_time(&self) -> Duration {
        self.frame_time
    }
}
