use std::ops::{Add, Div, Mul, Sub};

use num_traits::{real::Real, Float};

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x.lerp(other.x, t),
            y: self.y.lerp(other.y, t),
            z: self.z.lerp(other.z, t),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
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

// not ideal
pub trait Lerp<T> {
    fn lerp(&self, other: Self, t: T) -> Self;
}

impl Lerp<f32> for u8 {
    fn lerp(&self, other: Self, t: f32) -> Self {
        (*self as f32 + (*self as f32 - other as f32) * t) as u8
    }
}

impl Lerp<f32> for f32 {
    fn lerp(&self, other: Self, t: f32) -> Self {
        *self + (*self - other) * t
    }
}
