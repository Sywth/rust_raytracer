use std::f32::consts::PI;

pub trait Angle{
    fn to_radians(&self) -> Self;
    fn to_degrees(&self) -> Self;
}

impl Angle for f32{
    fn to_radians(&self) -> Self {
        return (self / 180.0) * PI ;
    }

    fn to_degrees(&self) -> Self {
        return (self / PI) * 180.0;
    }
}

// https://stackoverflow.com/a/59083859
pub fn inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - 0.5 * x * y * y)
}
