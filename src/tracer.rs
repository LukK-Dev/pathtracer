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

        for i in 0..self.image_buffer.len() {
            self.image_buffer[i] = rand::random();
        }

        self.frame_time = frame_time.elapsed();
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

    pub fn resize(&mut self, width: usize, height: usize) -> anyhow::Result<()> {
        if width < 1 || height < 1 {
            anyhow::bail!("Failed to resize tracer; size must be greater than zero!")
        }

        self.width = width;
        self.height = height;
        self.image_buffer.resize(width * height * 3, 255);
        Ok(())
    }

    pub fn frame_time(&self) -> Duration {
        self.frame_time
    }
}
