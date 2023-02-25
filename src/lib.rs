use std::{fs::File, io::Write, path::PathBuf};
mod math;
use math::Lerp;

pub fn save_buffer_to_ppm(
    path: PathBuf,
    buffer: &[u8],
    width: usize,
    height: usize,
) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(b"P6\n")?;
    file.write(&format!("{} {}\n", width, height).into_bytes())?;
    file.write_all(b"255\n")?;
    file.write_all(buffer)?;
    Ok(())
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 1,
    };
    pub const WHITE: Self = Self {
        r: 1,
        g: 1,
        b: 1,
        a: 1,
    };

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1 }
    }

    pub fn rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn rgb(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub fn lerp(&self, other: Color, t: f32) -> Self {
        Self {
            r: self.r.lerp(other.r, t),
            g: self.g.lerp(other.g, t),
            b: self.b.lerp(other.b, t),
            a: 1,
        }
    }
}
