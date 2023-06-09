use crate::vectorlib::vector3::Vector3f;

pub struct Color {
    pub vec: Vector3f,
}

pub struct Color24b {
    r: u8,
    g: u8,
    b: u8,
}

// use super::{Color, Color24b, Vector3f};
use std::{
    fmt::{self},
    io::Write,
};

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        return Color {
            vec: Vector3f::new(
                r.min(1.0).max(0.0),
                g.min(1.0).max(0.0),
                b.min(1.0).max(0.0),
            ),
        };
    }

    pub fn to_24_bit(&self) -> Color24b {
        return Color24b::new_from_f32(self.vec.x * 256.0, self.vec.y * 256.0, self.vec.z * 256.0);
    }

    pub fn from_vec(vec : Vector3f) -> Color{
        return Color {
            vec
        };
    }

    pub fn max_color() -> Color{
        return Color::new(1.0,1.0,1.0);
    }
    pub fn min_color() -> Color{
        return Color::new(0.0,0.0,0.0);
    }

    pub fn gamma_two_correct(&mut self) -> &Self{
        self.vec.x = self.vec.x.sqrt();
        self.vec.y = self.vec.y.sqrt();
        self.vec.z = self.vec.z.sqrt();
        return self;
    }
}

impl Color24b {
    pub fn new_from_f32(r: f32, g: f32, b: f32) -> Color24b {
        return Color24b {
            r: (r.min(255.0).max(0.0) as u8),
            g: (g.min(255.0).max(0.0) as u8),
            b: (b.min(255.0).max(0.0) as u8),
        };
    }

    pub fn write(&self, buffer: &mut dyn Write) {
        buffer
            .write_all(format!("{} {} {}\n", self.r, self.g, self.b).as_bytes())
            .unwrap();
    }
}

impl fmt::Display for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            formatter,
            "Color({},{},{})",
            self.vec.x, self.vec.y, self.vec.z
        );
    }
}

impl fmt::Display for Color24b {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "Color24b({},{},{})", self.r, self.g, self.b);
    }
}

pub trait ToColor {
    fn to_color(&self) -> Color;
}

impl ToColor for Vector3f{
    fn to_color(&self) -> Color {
        return Color::from_vec(self.clone());
    }
}