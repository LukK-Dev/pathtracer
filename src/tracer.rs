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
        const COLOR_1: [u8; 3] = [90, 122, 200];
        const COLOR_2: [u8; 3] = [190, 142, 240];
        let frame_time = Instant::now();

        for i in (0..self.image_buffer.len()).step_by(3) {
            let y = i / self.width;
            for j in 0..2 {
                self.image_buffer[i + j] =
                    lerp(COLOR_1[j], COLOR_2[j], y as f32 / self.height as f32)
            }
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

fn lerp(v1: u8, v2: u8, t: f32) -> u8 {
    (v1 as f32 + (v2 as f32 - v1 as f32) * t) as u8
}
